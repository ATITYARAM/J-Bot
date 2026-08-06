#include <chrono>
#include <vector>

#include "rclcpp/rclcpp.hpp"

#include "std_msgs/msg/int32.hpp"
#include "sensor_msgs/msg/laser_scan.hpp"

using namespace std::chrono_literals;

class Tilt : public rclcpp::Node
{
public:
    Tilt()
        : Node("tilt"),
          angle_(75)
    {
        angle_pub_ =
            create_publisher<std_msgs::msg::Int32>(
                "/jbot/tilt/angle",
                10);

        scan_pub_ =
            create_publisher<sensor_msgs::msg::LaserScan>(
                "/jbot/tilt/scan",
                10);

        scan_sub_ =
            create_subscription<sensor_msgs::msg::LaserScan>(
                "/jbot/scan",
                10,
                std::bind(
                    &Tilt::scan_callback,
                    this,
                    std::placeholders::_1));

        timer_ =
            create_wall_timer(
                100ms,
                std::bind(
                    &Tilt::publish_angle,
                    this));
    }

private:
    void publish_angle()
    {
        std_msgs::msg::Int32 msg;

        msg.data = angle_;

        angle_pub_->publish(msg);

        RCLCPP_INFO(
            get_logger(),
            "Angle -> %d",
            angle_);

        angle_++;

        if (angle_ > 105)
        {
            angle_ = 75;
        }
    }

    void scan_callback(
        const sensor_msgs::msg::LaserScan::SharedPtr msg)
    {
        sensor_msgs::msg::LaserScan front = *msg;

        front.ranges.clear();
        front.intensities.clear();

        const std::size_t total = msg->ranges.size();

        if (total == 0)
        {
            return;
        }

        // Front ±30°
        const std::size_t sector = total / 12;

        for (std::size_t i = total - sector; i < total; i++)
        {
            front.ranges.push_back(msg->ranges[i]);

            if (!msg->intensities.empty())
                front.intensities.push_back(msg->intensities[i]);
        }

        for (std::size_t i = 0; i < sector; i++)
        {
            front.ranges.push_back(msg->ranges[i]);

            if (!msg->intensities.empty())
                front.intensities.push_back(msg->intensities[i]);
        }

        scan_pub_->publish(front);
    }

    int angle_;

    rclcpp::Publisher<
        std_msgs::msg::Int32>::SharedPtr angle_pub_;

    rclcpp::Publisher<
        sensor_msgs::msg::LaserScan>::SharedPtr scan_pub_;

    rclcpp::Subscription<
        sensor_msgs::msg::LaserScan>::SharedPtr scan_sub_;

    rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    rclcpp::spin(
        std::make_shared<Tilt>());

    rclcpp::shutdown();

    return 0;
}

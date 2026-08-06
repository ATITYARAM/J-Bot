#include <cstdlib>

#include "rclcpp/rclcpp.hpp"
#include "sensor_msgs/msg/laser_scan.hpp"

class Lidar : public rclcpp::Node
{
public:
    Lidar()
        : Node("lidar")
    {
        RCLCPP_INFO(
            get_logger(),
            "Starting SLAMTEC RPLIDAR...");

        std::system(
            "ros2 launch sllidar_ros2 view_sllidar_a2m12_launch.py &");

        publisher_ =
            create_publisher<sensor_msgs::msg::LaserScan>(
                "/jbot/scan",
                10);

        subscription_ =
            create_subscription<sensor_msgs::msg::LaserScan>(
                "/scan",
                10,
                std::bind(
                    &Lidar::callback,
                    this,
                    std::placeholders::_1));

        RCLCPP_INFO(
            get_logger(),
            "Bridge: /scan -> /jbot/scan");
    }

private:
    void callback(
        const sensor_msgs::msg::LaserScan::SharedPtr msg)
    {
        publisher_->publish(*msg);
    }

    rclcpp::Publisher<
        sensor_msgs::msg::LaserScan>::SharedPtr publisher_;

    rclcpp::Subscription<
        sensor_msgs::msg::LaserScan>::SharedPtr subscription_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    rclcpp::spin(
        std::make_shared<Lidar>());

    rclcpp::shutdown();

    return 0;
}

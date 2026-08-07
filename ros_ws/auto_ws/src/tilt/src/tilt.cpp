#include <chrono>
#include <cmath>
#include <string>
#include <vector>

#include "rclcpp/rclcpp.hpp"

#include "std_msgs/msg/int32.hpp"
#include "std_msgs/msg/string.hpp"
#include "sensor_msgs/msg/laser_scan.hpp"
#include "std_msgs/msg/empty.hpp"

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

        snap_pub_ =
            create_publisher<std_msgs::msg::String>(
                "/jbot/tilt/snap",
                10);

        scan_sub_ =
            create_subscription<sensor_msgs::msg::LaserScan>(
                "/jbot/scan",
                10,
                std::bind(
                    &Tilt::scan_callback,
                    this,
                    std::placeholders::_1));

        start_sub_ =
    create_subscription<std_msgs::msg::Empty>(
        "/jbot/tilt/start",
        10,
        std::bind(
            &Tilt::start_callback,
            this,
            std::placeholders::_1));
    }

private:
    
    void start_callback(
    const std_msgs::msg::Empty::SharedPtr)
{
    if (running_)
    {
        return;
    }

    running_ = true;

    angle_ = 75;

    RCLCPP_INFO(
        get_logger(),
        "Tilt Started");
}

    void scan_callback(
    
        const sensor_msgs::msg::LaserScan::SharedPtr msg)
    {
    
        if (!running_)
        {
            return;
        }
        sensor_msgs::msg::LaserScan front = *msg;

        front.ranges.clear();
        front.intensities.clear();

        const std::size_t total = msg->ranges.size();

        if (total == 0)
        {
            return;
        }

        //--------------------------------------------------
        // Front ±30°
        //--------------------------------------------------

        const std::size_t sector = total / 12;

        for (std::size_t i = total - sector; i < total; i++)
        {
            front.ranges.push_back(msg->ranges[i]);

            if (!msg->intensities.empty())
            {
                front.intensities.push_back(msg->intensities[i]);
            }
        }

        for (std::size_t i = 0; i < sector; i++)
        {
            front.ranges.push_back(msg->ranges[i]);

            if (!msg->intensities.empty())
            {
                front.intensities.push_back(msg->intensities[i]);
            }
        }

        scan_pub_->publish(front);

        //--------------------------------------------------
        // Live Snapshot
        //--------------------------------------------------

        constexpr int ROWS = 10;
        constexpr int COLS = 15;

        constexpr float SAFE_DISTANCE = 0.35f;

        std::vector<std::string> matrix(
            ROWS,
            std::string(COLS, '0'));

        const std::size_t beams = front.ranges.size();

        for (std::size_t i = 0; i < beams; ++i)
        {
            float distance = front.ranges[i];

            if (!std::isfinite(distance))
            {
                continue;
            }

            if (distance > SAFE_DISTANCE)
            {
                continue;
            }

            int col =
                static_cast<int>(
                    (i * COLS) / beams);

            if (col >= COLS)
            {
                col = COLS - 1;
            }

            float ratio =
                distance / SAFE_DISTANCE;

            int occupied_rows =
                ROWS -
                static_cast<int>(ratio * ROWS);

            if (occupied_rows < 1)
            {
                occupied_rows = 1;
            }

            if (occupied_rows > ROWS)
            {
                occupied_rows = ROWS;
            }

            for (int r = ROWS - 1;
                 r >= ROWS - occupied_rows;
                 --r)
            {
                matrix[r][col] = '1';
            }
        }

        std_msgs::msg::String snap;

        for (int r = 0; r < ROWS; r++)
        {
            snap.data += matrix[r];

            if (r != ROWS - 1)
            {
                snap.data += '\n';
            }
        }

        snap_pub_->publish(snap);
        
        std_msgs::msg::Int32 angle;

        angle.data = angle_;

        angle_pub_->publish(angle);

        angle_++;

        if (angle_ > 105)
        {
            angle_ = 90;

            angle.data = 90;

            angle_pub_->publish(angle);

            running_ = false;

            RCLCPP_INFO(
                get_logger(),
                "Tilt Finished");
        }
    }

    int angle_;

    bool running_{false};

    rclcpp::Publisher<
        std_msgs::msg::Int32>::SharedPtr angle_pub_;

    rclcpp::Publisher<
        sensor_msgs::msg::LaserScan>::SharedPtr scan_pub_;

    rclcpp::Publisher<
        std_msgs::msg::String>::SharedPtr snap_pub_;

    rclcpp::Subscription<
        sensor_msgs::msg::LaserScan>::SharedPtr scan_sub_;

    rclcpp::Subscription<
        std_msgs::msg::Empty>::SharedPtr start_sub_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    rclcpp::spin(
        std::make_shared<Tilt>());

    rclcpp::shutdown();

    return 0;
}

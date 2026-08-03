#include <algorithm>
#include <cstdio>

#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/twist.hpp"
#include "std_msgs/msg/string.hpp"

class Viaduct : public rclcpp::Node
{
public:
    Viaduct()
        : Node("viaduct")
    {
        publisher_ =
            create_publisher<std_msgs::msg::String>(
                "/jbot",
                10);

        subscription_ =
            create_subscription<geometry_msgs::msg::Twist>(
                "/cmd_vel",
                10,
                std::bind(
                    &Viaduct::callback,
                    this,
                    std::placeholders::_1));

        RCLCPP_INFO(get_logger(), "J-BOT Viaduct Started");
    }

private:
    void callback(const geometry_msgs::msg::Twist::SharedPtr msg)
    {
        float linear  = msg->linear.x;
        float angular = msg->angular.z;

        float left  = linear - angular;
        float right = linear + angular;

        left  = std::clamp(left,  -1.0f, 1.0f);
        right = std::clamp(right, -1.0f, 1.0f);

        int left_pwm  = static_cast<int>(left * 255.0f);
        int right_pwm = static_cast<int>(right * 255.0f);

        char buffer[64];

        std::snprintf(
            buffer,
            sizeof(buffer),
            "L:%d R:%d",
            left_pwm,
            right_pwm);

        std_msgs::msg::String out;

        out.data = buffer;

        publisher_->publish(out);

        RCLCPP_INFO(
            get_logger(),
            "%s",
            buffer);
    }

    rclcpp::Publisher<std_msgs::msg::String>::SharedPtr publisher_;

    rclcpp::Subscription<geometry_msgs::msg::Twist>::SharedPtr subscription_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    rclcpp::spin(std::make_shared<Viaduct>());

    rclcpp::shutdown();

    return 0;
}

#include <chrono>

#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/twist.hpp"

using namespace std::chrono_literals;

class Teleop : public rclcpp::Node
{
public:
    Teleop()
        : Node("teleop")
    {
        publisher_ =
            create_publisher<geometry_msgs::msg::Twist>(
                "/cmd_vel",
                10);

        timer_ =
            create_wall_timer(
                1000ms,
                std::bind(&Teleop::publish_cmd, this));

        RCLCPP_INFO(get_logger(), "Teleop Started");
    }

private:
    void publish_cmd()
    {
        geometry_msgs::msg::Twist msg;

        msg.linear.x = 1.0;
        msg.angular.z = 0.0;

        publisher_->publish(msg);

        RCLCPP_INFO(get_logger(), "Publishing Forward");
    }

    rclcpp::Publisher<geometry_msgs::msg::Twist>::SharedPtr publisher_;
    rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char * argv[])
{
    rclcpp::init(argc, argv);

    rclcpp::spin(std::make_shared<Teleop>());

    rclcpp::shutdown();

    return 0;
}

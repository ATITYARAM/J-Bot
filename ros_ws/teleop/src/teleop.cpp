#include <termios.h>
#include <unistd.h>

#include "geometry_msgs/msg/twist.hpp"
#include "rclcpp/rclcpp.hpp"

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

        RCLCPP_INFO(
            get_logger(),
            "J-BOT Teleop Started");
        RCLCPP_INFO(
            get_logger(),
            "Controls:");
        RCLCPP_INFO(
            get_logger(),
            "  I - Forward");
        RCLCPP_INFO(
            get_logger(),
            "  K - Backward");
        RCLCPP_INFO(
            get_logger(),
            "  J - Left");
        RCLCPP_INFO(
            get_logger(),
            "  L - Right");
        RCLCPP_INFO(
            get_logger(),
            "Space - Stop");
        RCLCPP_INFO(
            get_logger(),
            "  Q - Quit");
    }

    void publish(const geometry_msgs::msg::Twist &msg)
    {
        publisher_->publish(msg);
    }

private:
    rclcpp::Publisher<geometry_msgs::msg::Twist>::SharedPtr publisher_;
};

char get_key()
{
    struct termios oldt;
    struct termios newt;

    tcgetattr(STDIN_FILENO, &oldt);

    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);

    tcsetattr(STDIN_FILENO, TCSANOW, &newt);

    char c = 0;

    read(STDIN_FILENO, &c, 1);

    tcsetattr(STDIN_FILENO, TCSANOW, &oldt);

    return c;
}

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    auto node = std::make_shared<Teleop>();

    while (rclcpp::ok())
    {
        char key = get_key();

        geometry_msgs::msg::Twist msg;

        switch (key)
        {
            case 'i':
            case 'I':
                msg.linear.x = 1.0;
                RCLCPP_INFO(node->get_logger(), "Forward");
                break;

            case 'k':
            case 'K':
                msg.linear.x = -1.0;
                RCLCPP_INFO(node->get_logger(), "Backward");
                break;

            case 'j':
            case 'J':
                msg.angular.z = 1.0;
                RCLCPP_INFO(node->get_logger(), "Left");
                break;

            case 'l':
            case 'L':
                msg.angular.z = -1.0;
                RCLCPP_INFO(node->get_logger(), "Right");
                break;

            case ' ':
                msg.linear.x = 0.0;
                msg.angular.z = 0.0;
                RCLCPP_INFO(node->get_logger(), "Stop");
                break;

            case 'q':
            case 'Q':
                rclcpp::shutdown();
                return 0;

            default:
                continue;
        }

        node->publish(msg);

        rclcpp::spin_some(node);
    }

    rclcpp::shutdown();

    return 0;
}

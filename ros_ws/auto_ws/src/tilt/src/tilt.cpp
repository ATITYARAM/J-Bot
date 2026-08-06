#include <chrono>

#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/int32.hpp"

using namespace std::chrono_literals;

class Tilt : public rclcpp::Node
{
public:
    Tilt()
        : Node("tilt"),
          angle_(75)
    {
        publisher_ =
            create_publisher<std_msgs::msg::Int32>(
                "/jbot/tilt/angle",
                10);

        timer_ =
            create_wall_timer(
                100ms,
                std::bind(
                    &Tilt::publish,
                    this));
    }

private:
    void publish()
    {
        std_msgs::msg::Int32 msg;

        msg.data = angle_;

        publisher_->publish(msg);

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

    int angle_;

    rclcpp::Publisher<
        std_msgs::msg::Int32>::SharedPtr publisher_;

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

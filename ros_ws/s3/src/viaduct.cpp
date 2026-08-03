#include <fcntl.h>
#include <unistd.h>
#include <termios.h>

#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"

class Viaduct : public rclcpp::Node
{
public:
    Viaduct()
        : Node("viaduct")
    {
        serial_ = open("/dev/ttyACM0", O_RDWR | O_NOCTTY);

        if (serial_ < 0)
        {
            RCLCPP_ERROR(get_logger(), "Cannot open /dev/ttyACM0");
        }
        else
        {
            struct termios tty {};

            tcgetattr(serial_, &tty);

            cfsetispeed(&tty, B115200);
            cfsetospeed(&tty, B115200);

            tty.c_cflag |= (CLOCAL | CREAD);
            tty.c_cflag &= ~CSIZE;
            tty.c_cflag |= CS8;
            tty.c_cflag &= ~PARENB;
            tty.c_cflag &= ~CSTOPB;
            tty.c_cflag &= ~CRTSCTS;

            tty.c_lflag = 0;
            tty.c_iflag = 0;
            tty.c_oflag = 0;

            tcsetattr(serial_, TCSANOW, &tty);

            RCLCPP_INFO(
                get_logger(),
                "Connected to /dev/ttyACM0");
        }

        subscription_ =
            create_subscription<std_msgs::msg::String>(
                "/jbot",
                10,
                std::bind(
                    &Viaduct::callback,
                    this,
                    std::placeholders::_1));
    }

    ~Viaduct()
    {
        if (serial_ >= 0)
        {
            close(serial_);
        }
    }

private:
    void callback(const std_msgs::msg::String::SharedPtr msg)
    {
        RCLCPP_INFO(
            get_logger(),
            "TX -> %s",
            msg->data.c_str());

        if (serial_ >= 0)
        {
            write(
                serial_,
                msg->data.c_str(),
                msg->data.size());

            write(serial_, "\n", 1);
        }
    }

    int serial_{-1};

    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr subscription_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    rclcpp::spin(
        std::make_shared<Viaduct>());

    rclcpp::shutdown();

    return 0;
}

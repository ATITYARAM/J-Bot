#include <fcntl.h>
#include <unistd.h>
#include <termios.h>

#include <filesystem>
#include <fstream>
#include <string>

#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"

namespace fs = std::filesystem;

class Viaduct : public rclcpp::Node
{
public:
    Viaduct()
        : Node("viaduct")
    {
        std::string port = find_esp_port();

        if (port.empty())
        {
            RCLCPP_ERROR(get_logger(), "No ESP32 / ESP32-S3 Found");
            return;
        }

        serial_ = open(port.c_str(), O_RDWR | O_NOCTTY);

        if (serial_ < 0)
        {
            RCLCPP_ERROR(get_logger(), "Cannot open %s", port.c_str());
            return;
        }

        struct termios tty{};

        tcgetattr(serial_, &tty);

        cfsetispeed(&tty, B115200);
        cfsetospeed(&tty, B115200);

        tty.c_cflag |= (CLOCAL | CREAD);
        tty.c_cflag &= ~CSIZE;
        tty.c_cflag |= CS8;
        tty.c_cflag &= ~PARENB;
        tty.c_cflag &= ~CSTOPB;
        tty.c_cflag &= ~CRTSCTS;

        tty.c_iflag = 0;
        tty.c_oflag = 0;
        tty.c_lflag = 0;

        tcsetattr(serial_, TCSANOW, &tty);

        RCLCPP_INFO(
            get_logger(),
            "Connected -> %s",
            port.c_str());

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
            close(serial_);
    }

private:

    std::string find_esp_port()
    {
        const std::string root = "/sys/class/tty";

        for (const auto &entry : fs::directory_iterator(root))
        {
            std::string tty = entry.path().filename();

            if (tty.rfind("ttyACM", 0) != 0 &&
                tty.rfind("ttyUSB", 0) != 0)
                continue;

            fs::path device = fs::canonical(entry.path() / "device");

            fs::path usb = device;

            while (!usb.empty())
            {
                if (fs::exists(usb / "idVendor") &&
                    fs::exists(usb / "idProduct"))
                {
                    std::ifstream vendor(usb / "idVendor");
                    std::ifstream product(usb / "idProduct");

                    std::string vid;
                    std::string pid;

                    vendor >> vid;
                    product >> pid;

                    // Espressif VID
                    if (vid == "303a")
                    {
                        return "/dev/" + tty;
                    }

                    break;
                }

                usb = usb.parent_path();
            }
        }

        return "";
    }

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

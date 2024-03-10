
#![no_std]
use cpp::cpp;

cpp! {{
    #include <Arduino.h>
    #include <GyverOLED.h>
}}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Entry point

#[no_mangle]
extern "C" fn app_main() {
    let name_ptr=5;
    let x = unsafe {
        cpp!([name_ptr as "int"] -> u32 as "int32_t" {
            initArduino();
            HardwareSerial serial(0);
            serial.begin(115200);
            serial.write("abcd");
            serial.flush();

            GyverOLED<SSD1306_128x64, OLED_NO_BUFFER, OLED_I2C> oled;
            oled.init();  // инициализация
            oled.clear();
            oled.home();           // курсор в 0,0
            oled.setScale(3);
            oled.print("abcd");   // печатай что угодно: числа, строки, float, как Serial!
            oled.update();
            return 42;
        })
    };

}

extern crate cpp_build;

fn main() {
    let lib_path = ".pio/build/debug";
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=libFrameworkArduino");
    cpp_build::Config::new()
    .compiler("/home/mike/.platformio/packages/toolchain-xtensa-esp32/bin/xtensa-esp32-elf-gcc")
    .flag("-std=gnu99")
    .flag("-fno-rtti")
    .define("PLATFORMIO", Some("60113"))
    .define("ARDUINO_ESP32_DEV", None)
    .define("HAVE_CONFIG_H", None)
    .define("MBEDTLS_CONFIG_FILE", Some("mbedtls/esp_config.h"))
    .define("UNITY_INCLUDE_CONFIG_H", None)
    .define("WITH_POSIX", None)
    .define("_GNU_SOURCE", None)
    .define("IDF_VER" , Some("v4.4.3"))
    .define("ESP_PLATFORM", None)
    .define("_POSIX_READER_WRITER_LOCKS", None)
    .define("ARDUINO_ARCH_ESP32", None)
    .define("ESP32", None)
    .define("F_CPU", Some("240000000L"))
    .define("ARDUINO", Some("10812"))
    .define("ARDUINO_VARIANT", Some("esp32"))
    .define("ARDUINO_BOARD", Some("Espressif ESP-WROVER-KIT"))
    .define("ARDUINO_PARTITION_default", None)
    .define("__PLATFORMIO_BUILD_DEBUG__", None)
    .include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/newlib/platform_include")
    .include("src")
    .include(".pio/libdeps/debug/GyverOLED/src")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/freertos/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/freertos/include/esp_additions/freertos")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/freertos/port/xtensa/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/freertos/include/esp_additions")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hw_support/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hw_support/include/soc")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hw_support/include/soc/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hw_support/port/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hw_support/port/esp32/private_include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/heap/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/log/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/lwip/include/apps")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/lwip/include/apps/sntp")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/lwip/lwip/src/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/lwip/port/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/lwip/port/esp32/include/arch")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/soc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/soc/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/soc/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/hal/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/hal/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/hal/platform_port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_rom/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_rom/include/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_rom/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_system/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_system/port/soc")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_system/port/public_compat")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/xtensa/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/xtensa/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/driver/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/driver/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_pm/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_ringbuf/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/efuse/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/efuse/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/vfs/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_wifi/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_event/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_netif/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_eth/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/tcpip_adapter/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_phy/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_phy/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_ipc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/app_trace/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_timer/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/mbedtls/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/mbedtls/mbedtls/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/mbedtls/esp_crt_bundle/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/app_update/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/spi_flash/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bootloader_support/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/nvs_flash/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/pthread/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_gdbstub/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_gdbstub/xtensa")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_gdbstub/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/espcoredump/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/espcoredump/include/port/xtensa")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/wpa_supplicant/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/wpa_supplicant/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/wpa_supplicant/esp_supplicant/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/ieee802154/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/console")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/asio/asio/asio/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/asio/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/common/osi/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/include/esp32/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/common/api/include/api")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/common/btc/profile/esp/blufi/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/common/btc/profile/esp/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/host/bluedroid/api/include/api")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_common/tinycrypt/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_core")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_core/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_core/storage")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/btc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_models/common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_models/client/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/mesh_models/server/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/api/core/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/api/models/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/bt/esp_ble_mesh/api")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/cbor/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/unity/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/unity/unity/src")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/cmock/CMock/src")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/coap/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/coap/libcoap/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/nghttp/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/nghttp/nghttp2/lib/includes")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-tls")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-tls/esp-tls-crypto")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_adc_cal/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_hid/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/tcp_transport/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_http_client/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_http_server/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_https_ota/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_https_server/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_lcd/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_lcd/interface")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/protobuf-c/protobuf-c")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/protocomm/include/common")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/protocomm/include/security")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/protocomm/include/transports")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/mdns/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_local_ctrl/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/sdmmc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_serial_slave_link/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_websocket_client/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/expat/expat/expat/lib")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/expat/port/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/wear_levelling/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/fatfs/diskio")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/fatfs/vfs")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/fatfs/src")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/freemodbus/common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/idf_test/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/idf_test/include/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/jsmn/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/json/cJSON")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/libsodium/libsodium/src/libsodium/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/libsodium/port_include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/mqtt/esp-mqtt/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/openssl/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/perfmon/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/spiffs/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/ulp/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/wifi_provisioning/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/rmaker_common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/json_parser/upstream/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/json_parser/upstream")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/json_generator/upstream")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_schedule/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_rainmaker/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/gpio_button/button/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/qrcode/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/ws2812_led")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_diagnostics/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/rtc_store/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_insights/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/dotprod/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/support/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/hann/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/blackman/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/blackman_harris/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/blackman_nuttall/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/nuttall/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/windows/flat_top/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/iir/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/fir/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/add/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/sub/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/mul/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/addc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/mulc/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/math/sqrt/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/matrix/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/fft/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/dct/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/conv/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/common/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/kalman/ekf/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dsp/modules/kalman/ekf_imu13states/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp_littlefs/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/tool")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/typedef")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/image")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/math")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/nn")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/layer")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/detect")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-dl/include/model_zoo")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-sr/src/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-sr/esp-tts/esp_tts_chinese/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp-sr/include/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp32-camera/driver/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/esp32-camera/conversions/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/include/fb_gfx/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/tools/sdk/esp32/dio_qspi/include")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/cores/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/variants/esp32")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/libraries/Wire/src")
.include("/home/mike/.platformio/packages/framework-arduinoespressif32/libraries/SPI/src")
    .build("src/lib.rs");
}
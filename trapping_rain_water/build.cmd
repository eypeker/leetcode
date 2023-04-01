:: cargo build --target wasm32-unknown-unknown -j 16 --release
wasm-pack build
timeout 1

:: copy ".\target\wasm32-unknown-unknown\release\trapping_rain_water.wasm" ".\www\trapping_rain_water.wasm"
:: copy ".\target\wasm32-unknown-unknown\release\trapping_rain_water.wasm" "\\wsl$\Ubuntu\home\blueinvader\trapping_rain_water.wasm"
copy ".\pkg\trapping_rain_water_bg.wasm" ".\www\trapping_rain_water.wasm"

timeout 1

:: rmdir /s /q ".\target\"



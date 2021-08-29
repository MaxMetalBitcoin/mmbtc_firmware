Hello! This is a repo to hold the firmware for building a bitcoin-only hardware wallet.

Current HW Requirements:
- STM32F3 Discovery
- SD Card Reader

Will need in future:
- Auto-focus camera
- Display
- Either Capacitive touch or (up, down, left, right, select buttons)

`/hw_discovery` holds code to run this firmware on an STM32F3 Discovery device.
`/state_mgmt` holds code to manage the state of the app - state updates will then be used to impact 'effects' ie the display, sd card, etc.
`/simulator` holds code to run a simulated version of the hww on a mac
`full_app_test` holds code to play around with RTIC to run the app. Not sure I like it

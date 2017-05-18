# pigpio
Set or Read Raspberry Pi3 GPIO pins

# installation from github:
  1. first download the snap:
    * manual from https://github.com/VBota1/pi3gpio/tree/master/snap
    * or by cloning repository: ```git clone https://github.com/VBota1/pi3gpio```
  2. then navigate to the folder containing the snap
  3. then run:
    ```sudo snap install --devmode pi3gpio_1.8_armhf.snap```

# installation from ubuntu store
  ```sudo snap install pi3gpio --devmode --channel=candidate```
        
# description:
  Use this app to set or read Raspberry Pi3 GPIO pins from your console.

  Examples without snap:   
  ```
    sudo ./pigpio help    
    sudo ./pigpio get all     
    sudo ./pigpio get 4   
    sudo ./pigpio get 10 11   
    sudo ./pigpio set low all   
    sudo ./pigpio set high 12
    sudo ./pigpio set low 2 5 7
  ```
  Examples with snap:   
  ```
    sudo pigpio help    
    sudo pigpio get all   
    sudo pigpio get 4   
    sudo pigpio get 10 11   
    sudo pigpio set low all   
    sudo pigpio set high 12
    sudo pigpio set low 2 5 7
  ```

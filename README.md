# pigpio
Set or Read Raspberry Pi3 GPIO pins

# installation from github:
  1. first download the snap:
    * manual from https://github.com/VBota1/pi3gpio/snap
    * or by cloning repository: ```git clone https://github.com/VBota1/pi3gpio```
  2. then navigate to the folder containing the snap
  3. then run:
    ```sudo snap install --dangerous pi3gpio_2.4_armhf.snap```

# installation from ubuntu store
  ```sudo snap install pi3gpio```

# GPIO access
  The application needs access to /dev/mem or /dev/gpiomem to control GPIO pins. That is why sudo is needed.
  In order to gain access to /dev/mem run, only once after installation, the following commands:
  ```
    sudo snap connect pi3gpio:physical-memory-control core:physical-memory-control
    sudo snap connect pi3gpio:physical-memory-observe core:physical-memory-observe
  ```

# description:
  Use this app to set or read Raspberry Pi3 GPIO pins from your console.  
  ```
    sudo pigpio help    
    sudo pigpio get all   
    sudo pigpio get 4   
    sudo pigpio get 10 11   
    sudo pigpio set low all   
    sudo pigpio set high 12
    sudo pigpio set low 2 5 7
  ```

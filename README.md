# pigpio
Set or Read Raspberry Pi3 GPIO pins

# installation with snap:
'       first download the snap packet from the folder pigpio/snap
        then run:

                sudo snap install --devmode pigpio_1.1_armhf.snap

# description:
'       Use this app to set or read Raspberry Pi3 GPIO pins from your console.
        Usage: sudo [./]pigpio command [state] [pins]   
                command:
                        help    prints the help text 
                        set     sets the value of the indicated pins to the indica$
                        get     prints the state of the indicated pins           
                state:
                        high    logical 1 for the indicated pins, equivalent volta$
                        low     logical 0 for the indicated pins, equivalent volta$
                pins:
                        all     BCM pins between 0 and 27 
        Examples without snap:   

                sudo ./pigpio help    
                sudo ./pigpio get all   
                sudo ./pigpio get 4   
                sudo ./pigpio get 10 11   
                sudo ./pigpio set low all   
                sudo ./pigpio set high 12
                sudo ./pigpio set low 2 5 7

'        Examples with snap:   

                sudo pigpio help    
                sudo pigpio get all   
                sudo pigpio get 4   
                sudo pigpio get 10 11   
                sudo pigpio set low all   
                sudo pigpio set high 12
                sudo pigpio set low 2 5 7

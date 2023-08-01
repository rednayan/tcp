##### important links
[tun-tap](https://docs.rs/tun-tap/latest/tun_tap/)

[tun-tap-document](https://www.kernel.org/doc/Documentation/networking/tuntap.txt)

[rfc-793](https://datatracker.ietf.org/doc/html/rfc793)

###### set the permissions for initializing tun interface
``` bash 
    sudo setcap cap_net_admin=eip target/release/tcp
```

###### set the ip for the tun interface
``` bash 
    sudo ip addr add 192.168.0.1/24 dev tun0
```

###### bring up interface 
``` bash 
    sudo ip link set up dev tun0
```

###### ping the interface(tun0) 
``` bash 
    ping -I tun0 192.168.0.2
```

###### watch the interface(tun0), reads bytes coming to the addr --- using tshark 
``` bash 
    tshark -i tun0
```
--38.55--

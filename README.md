<h1>WiOn SmartSwitch/Outlet (KAB enterprises protocol) Implementation</h1>

<img src="https://github.com/superhac/wion/blob/master/docs/imgs/wion_switch.jpg" align=right alt="WiOn Switch" style="width:150px;height:228px;">

<p>This is a very early implementation of KAB enterprises protocol in Rust that is used to control smart plugs and switches.  They are sold under numerous brand names such as WiOn or ECOplugs.</p>
<h2> Device Discovery </h2>
<p>  The discovery of devices on the network is achieved by sending a UDP broadcast packet on either port 25 or 5888 to the local network where the device(s) reside. </p>
<h3>Discovery Request</h3>
<p>The payload of this broadcast is comprised of 128 bytes all set to zero except for six bytes starting at offset 24 with the following values, 0xE0, 0x07, 0x06, 0x07, 0x07, 0xE0.  As far I've been able to surmise these bytes don't represent anything tangible, but they must be represented exactly as specified starting at offset 24.  Below is a Hex dump representation of the payload: <BR><BR>
<pre width="30">
0000   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0010   00 00 00 00 00 00 00 00 e0 07 06 07 07 e6 00 00
0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0040   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0050   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0060   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0070   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
</pre>
</p>
<h3> Discovery Response </h3>
<p>The following is the response structure that will be returned from each device after the broadcast has been sent:
<pre>
pub struct BroadcastResp {
    pub unknown: u32,
    pub version: [u8;6], //string
    pub dev_model: [u8;32],   //string
    pub dev_name: [u8;32],  //string
    pub dev_serial: [u8;34],  //string
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub ssid: [u8;64],  //string
    pub wifi_pass: [u8;64],  //string
    pub unknown5: u32,
    pub unknown6: u32,
    pub unknown7: u32,
    pub zipcode: [u8;12],
    pub p2pm: [u8;16],  //string ip
    pub p2ps: [u8;16], //string ip
    pub paw: [u8;16], //string ip
    pub unknown8: u32,
    pub unknown9: u32,
    pub unknown10: u32,
    pub unknown11: u32,
    pub unknown12: u32,
    pub unknown13: u32,
    pub unknown14: u32,
    pub unknown15: u32,
    pub unknown16: u32,
    pub unknown17: u32,
    pub unknown18: u32,
    pub unknown19: u32,
    pub dev_mac: [u8;18], //string
    pub dev_ip: [u8;18], //string
    pub dev_port: u32,
  }
</pre>  
The total byte size of the response is always 408 bytes (because the max size of all dynamic fields is fixed.  E.g dev_name is set to hard 32 byte length). You should also note that response includes the WiFi SSID the device is connected to and the WiFi password of that network.  <b>Both values are transmitted in clear text.</b>  Below is an example of a parsed response:
<pre>
Unknown: 0x0
Version: 1.6.0
Model: ECO-74227201
Name: Basement
Serial: 74227201
Unknown2: 0x150117E1
Unknown3: 0x9FF0
Unknown4: 0xFFFF3B20
SSID: myhouse
WiFi Pass: mywifipass
Unknown5: 0x30300001
Unknown6: 0x31
Unknown7: 0x0
Zipcode: 90210
p2pm: 210.71.198.37
p2ps: 210.71.198.37
paw: 61.220.255.143
Unknown8: 0x1
Unknown9: 0x2
Unknown10: 0x3
Unknown11: 0x10101
Unknown12: 0x43530001
Unknown13: 0x4C347A71
Unknown14: 0x64434676
Unknown15: 0x0
Unknown16: 0x0
Unknown17: 0x0
Unknown18: 0x0
Unknown19: 0x0
MAC: 48:5c:2a:4d:a1:22
IP: 192.168.0.248
Port: 80
</pre>
</p>

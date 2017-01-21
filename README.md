<h1>WiOn SmartSwitch/Outlet (KAB enterprises protocol) Implementation</h1>

<img src="https://github.com/superhac/wion/blob/master/docs/imgs/wion_switch.jpg" align=right alt="WiOn Switch" style="width:150px;height:228px;">

<p>This is a very early implementation of KAB enterprises protocol in Rust that is used to control smart plugs and switches.  They are sold under numerous brand names such as WiOn or ECOplugs.</p>
<h2> Device Discovery </h2>
<p>  The discovery of devices on the network is achieved by sending a UDP broadcast packet on either port 25 or 5888 to the local network where the device(s) reside.  The payload of this broadcast is comprised of 128 bytes all set to zero except for six bytes starting at offset 24 with the following values, 0xE0, 0x07, 0x06, 0x07, 0x07, 0xE0. </p>    

<h1>WiOn SmartSwitch/Outlet (KAB enterprises protocol) Implementation</h1>

<img src="https://github.com/superhac/wion/blob/master/docs/imgs/wion_switch.jpg" align=right alt="WiOn Switch" style="width:150px;height:228px;">

<p>This is a very early implementation of KAB enterprises protocol in Rust that is used to control smart plugs and switches.  They are sold under numerous brand names such as WiOn or ECOplugs.</p>
<h2> Device Discovery </h2>
<p>  The discovery of devices on the network is achieved by sending a UDP broadcast packet on either port 25 or 5888 to the local network where the device(s) reside.  The payload of this broadcast is comprised of 128 bytes all set to zero except for six bytes starting at offset 24 with the following values, 0xE0, 0x07, 0x06, 0x07, 0x07, 0xE0.  As far I've been able to surmise these bytes don't represent anything tangible, but they must be represented exactly as specified starting at offset 24.  Below is a Hex dump representation of the payload: <BR><BR>
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

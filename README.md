<h1>WiOn SmartSwitch/Outlet (KAB enterprises protocol) Implementation</h1>

<img src="https://github.com/superhac/wion/blob/master/docs/imgs/wion_switch.jpg" align=right alt="WiOn Switch" style="width:150px;height:228px;">

<p>This is a very early implementation of KAB enterprises protocol in Rust that is used to control smart plugs and switches.  They are sold under numerous brand names such as WiOn or ECOplugs.</p>
<h2> Device Discovery </h2>
<p>  The discovery of devices on the network is achieved by sending a UDP broadcast packet on either port 25 or 5888 to the local network where the device(s) reside. </p>
<h3>Discovery Request</h3>
<p>The payload of this broadcast is comprised of 128 bytes all set to zero except for six bytes starting at offset 24 with the following values, 0xE0, 0x07, 0x06, 0x07, 0x07, 0xE0.  As far I've been able to surmise these bytes don't represent anything tangible, but they must be represented exactly as specified starting at offset 24.  Below is a Hex dump representation of the payload:
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
<h2>Request/Response Header</h2>
<p>The request header is the common header to all directed communication with the device.  Note that the last two bytes of the structure are only present on "Requests".  Thus the base size of a request is 46 bytes, while the base response packet size is 44 bytes.  
<pre>
pub struct Header {
    pub cmd: u32,
    pub req_conn_id: u32,
    pub cmd_type: u16,
    pub version: [u8;6], //Convert to String
    pub model: [u8;32],  //Convert to String
    pub dev_name: [u8;32],  //Convert to String
    pub serial: [u8;32],  //Convert to String
    pub resp_status: u32,
    pub seq_counter: u32,
    pub unknown: u32,
    pub resp_conn_id: u32,
    pub operation: u8, // read or write? Field only on requests.  Field only seen on requests.
    pub rw_byte: u8, // what byte to read or write to?  Field only on requests.
}
</pre></p>
<h2>Known Commands</h2>
<p>Below is a list of the known commands:
public static final int CMD_BASCI_CONNECT_ROUTER = 327697;
public static final int CMD_BASCI_END_UPGRADE = 328195;
public static final int CMD_BASCI_GET_RDM_STATUS = 327713;
public static final int CMD_BASCI_GET_SETTING = 327685;
public static final int CMD_BASCI_GET_SWITCH_STATUS = 327703;
public static final int CMD_BASCI_GET_WIFI_HOT = 327698;
public static final int CMD_BASCI_HEART_PAKG = 262144;
public static final int CMD_BASCI_MODIFY_ALIAS_PSW = 327699;
public static final int CMD_BASCI_MODIFY_AREAINFOR = 327716;
public static final int CMD_BASCI_MODIFY_IP_PORT = 327700;
public static final int CMD_BASCI_MODIFY_SWITCH = 327702;
public static final int CMD_BASCI_MODIFY_TIMEZONE = 327701;
public static final int CMD_BASCI_NIGHTLAMP_SETTING = 327715;
public static final int CMD_BASCI_POST_ASTROMICTABLE = 327704;
public static final int CMD_BASCI_POST_SETTING = 327684;
public static final int CMD_BASCI_REDA_PWR_OFFSET = 327730;
public static final int CMD_BASCI_RESTART = 327682;
public static final int CMD_BASCI_RESTORE = 327683;
public static final int CMD_BASCI_SCHEDULE_RANDON = 327714;
public static final int CMD_BASCI_SET_RDM_STATUS = 327712;
public static final int CMD_BASCI_START_UPGRADE = 328193;
public static final int CMD_BASCI_WRITE_PWR_OFFSET = 327731;
public static final int CMD_FAILED = 1;
public static final int CMD_GET_SETTING_STATU = 327941;
public static final int CMD_GET_TODAY_TASKTAB = 327940;
public static final int CMD_NOPERMIT = 3;
public static final int CMD_REDO = 2;
public static final int CMD_RESPONSE_FAILED = 2;
public static final int CMD_RESPONSE_NOPERMIT = 3;
public static final int CMD_RESPONSE_SUCCESS = 1;
public static final int CMD_RESPONSE_TIMEOUT = 4;
public static final int CMD_RESPONSE_WAIT = 5;
public static final int CMD_SCHEDULE_ADD = 327936;
public static final int CMD_SCHEDULE_DELETE = 327938;
public static final int CMD_SCHEDULE_EDIT = 327937;
public static final int CMD_SCHEDULE_GETALL = 327939;
public static final int CMD_SUCCEDED = 0;
public static final int CMD_WAIT = 6;
public static final int DEVICE_SMART_SN_LENGTH = 20;
public static final int DVS_SEARCH_REQ_ALL = 1;
public static final int DVS_SEARCH_REQ_ONE = 0;
public static final int MAX_TASKNUMS = 20;
public static final int MAX_WIFINUMS = 10;
public static final int SMARTCTRL_BROADCAST_PORT = 5888;
public static final int SMARTCTRL_BROADCAST_PORT2 = 25;
public static final int SMARTCTRL_DEFAULT_PORT = 6000;
public static final int SMARTCTRL_UPGRADE_PORT = 1932;
public static final String SMARTDEFINE_VERSION = "FSD1.0";
public static final int UM_MESSAGE_DEVUPDATE = 1605633;
public static final int UM_MESSAGE_DEVUPDATESTATUS = 1605636;
public static final int UM_MESSAGE_GETFIRMWARELIST = 1605640;
public static final int UM_MESSAGE_GETPOWERINFOFAILED = 1605654;
public static final int UM_MESSAGE_GETPOWERINFOSUCCESS = 1605653;
public static final int UM_MESSAGE_GETRDMSETTING = 1605639;
public static final int UM_MESSAGE_GETSETTINGFAILED = 1605648;
public static final int UM_MESSAGE_GETSETTINGSUCCESS = 1605641;
public static final int UM_MESSAGE_STARTUPDATEFAILED = 1605651;
public static final int UM_MESSAGE_STARTUPDATEFIRMWARE = 1605649;
public static final int UM_MESSAGE_STARTUPDATESUCCESS = 1605650;
public static final int UM_MESSAGE_TODAYSCHEUPDATE = 1605655;
public static final int UM_MESSAGE_UPDATEFIRMWAREEND = 1605652;
public static final int UM_MESSAGE_WIFIUPDATE = 1605634;
</pre>
</p>

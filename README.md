<h1>WiOn SmartSwitch/Outlet (KAB enterprises protocol) Implementation</h1>

<img src="https://github.com/superhac/wion/blob/master/docs/imgs/wion_switch.jpg" align=right alt="WiOn Switch" style="width:150px;height:228px;">

<p>This is a very early implementation of KAB enterprises protocol in Rust that is used to control smart plugs and switches.  They are sold under numerous brand names such as WiOn or ECOplugs.</p>
<h2>Endianness</h2>
<p>All multi-byte numerical's are transmitted in little endian order.
</p>
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
<h2>Basic Request/Response Header</h2>
<p>The basic request/response header is the common header to all communication with the device.
<pre>
pub struct Header {
    pub cmd: u32,
    pub req_conn_id: u32,
    pub cmd_type: u16,
    pub version: [u8;6], //Convert to String - Fixed length (unused bytes are nulled, 0x00)
    pub model: [u8;32],  //Convert to String - Fixed length (unused bytes are nulled, 0x00)
    pub dev_name: [u8;32],  //Convert to String - Fixed length (unused bytes are nulled, 0x00)
    pub serial: [u8;32],  //Convert to String - Fixed length (unused bytes are nulled, 0x00)
    pub resp_status: u32,
    pub seq_counter: u32,
    pub unknown: u32,
    pub resp_conn_id: u32,
}
</pre></p>
<h2>Toggle Switch On or Off</h2>
<p>The switch can be turned by loading and transmitting the following structure to the device:
<pre>
//Header minimum fields
head.cmd = 327702;
head.req_conn_id = rng.gen::<u32>(); ; // needs to be changed each time or device is flaky with fast changes.  
                                          using rand now,
head.cmd_type = 0x02;
// must have model or the  device will not on turn
head.model = [0x45, 0x43, 0x4F, 0x2D, 0x37, 0x38, 0x30, 0x30, 0x34, 0x42, 0x30, 0x31, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00];  // translates to "ECO-78004B01" with null pads.
head.seq_counter = 0x55555555; // should be incremented, but it doesn't really matter, used for tracking

//additional fields after the basic "header"
head.operation = 0x02;
head.rw_byte = 1;  // 1 = on, 0 = off  
</pre>
You'll notice that certain fields within the <b>Header</b> structure need not be set.  These are the minimum required for the switch to be turned on and off.  Its important that
you set the <b>model</b> field to your specific device.  The <b>model</b> field is a combination of "ECO-" and the serial number of your device.  Use the discovery packet to identified
this field.  There are two additional fields that are added to the basic <b>Header</b>.   They are are <b>operation</b> and <b>rw_byte</b>.  The <b>operation</b> field specifies the operation as read or write.  These are signified by 0x02 for write and 0x00 for read. The <b>rw_byte</b> is the value that is to be written.  In this case writing 0 means switch off and 1 means switch on.  Below is an example response:
you will receive from the device (128 bytes):
<pre>
[Cmd: 0x50016, Req Conn ID: 0x84DD0000, cmd_type: 0x0,
 Version: 1.6.0, Model: ECO-78004B01, Dev_name: Basement test, Serial: 78004B01,
 Resp_Status: 0x7E11DC5E, Seq Counter: 1431655765, Unknown: 0 Resp Conn ID: 0x0]
</pre>
The way you toggle the switch off is to use the same populated <b>Header</b> structure as above, except you set the <b>rwByte</b> field to 0 as shown below:
<pre>
head.rw_byte = 0;  // 1 = on, 0 = off
</pre>
Note that the last two bytes (operation, rw_bytes) of the structure are only present on "Requests".  Thus the base size of a request is 130 bytes, while the base response packet size is 128 bytes.</p>
<h2>Scheduling</h2>
<p>These devices contain the ability to autonomously manage set points for turning on and off at specified times. The WiOn product has the ability to store 10 schedules per device.  Other Kab protocol based devices may have more or less.   The header for scheduling is the same as the basic <b>Header<b> with the following additional fields:
<pre>
tableEntryCount: u8, // contains the number of populated "tableEntryStructs" that are following
entryNum: u8,
unknown: u16,
counterType: u8  // the type of counter type.  See Appendix
</pre>
After this preamble header you get into the scheduling specific data structures.  The first structure is <b>tableEntry</b> and then its followed n (max of 9) number of <b>tableEntryNext</b> structures.  The reason that theres two structure's is the <b>unknown/b> field in <b>tableEntry</b> is four bytes while the same <b>unknown</b> field in <b>tableEntryNext</b> is only a single byte.  I have yet to figure out this discrepancy.
<pre>
//tableEntry structure
daysOfTheWeek: u8,
unknownBlob: [u8;8],
startYear: u16, // the year.  E.g the start year of timmr. eg. 2017
startMonth: u8, // month number 1-12
startDay: u8, // day of the month.  1-31
startTimeInSecs: u32, // the time the switch should turn on in seconds that represent military time.  E.g. 75600 = 21:00 hours
unknown: u16,
endYear: u16, // the year.  E.g the end year of timer. eg. 2017
endMonth: u8, // end month number 1-12
endDay: u8, // end day of the month.  1-31   
unknown: u16,
endTimeInSecs: u32 // // the time the switch should turn off in seconds that represent military time.  E.g. 75600 = 21:00 hours
</pre>

</p>
<h2>Appendix</h2>
<h3>Known Commands</h3>
<p>Below is a list of the known commands:
<pre>
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
<h3>daysOfTheWeek</h2>
<p>
The field within the <b>schedule</b> structure specified by <b>daysOfTheWeek</b> (u8) is decoded with the following bit masks:
<pre>
Sunday = 0x40
Monday = 0x20
Tuesday = 0x10
Wednesday = 0x08
Thursday = 0x04
Friday = 0x02
Saturday = 0x01
</pre>
Since this is a mask, multiple values are possible.   E.g. Monday, Tuesday, and Saturday can be represented within the byte,  
</p>
<h3>counterType</h2>
<p>Schedules have diffferent counter types.  These are the known ones:
<pre>
programable_timer = 0x02
countDownTimer = 0x00;
</pre>
</p>

# LNsploit

A Lightning Network exploit toolkit.

## Feature Ideas

- UI:

  - Based on [tui-rs](https://github.com/fdehau/tui-rs)
  - A network explorer view of some sort
    - 20k+ nodes is a lot.. maybe do some filtering such as large value channels only?
    - Color code nodes that are owned by you, maybe have a 6 hop view of nodes you can reach easily
  - A list of nodes you own, since in LDK there can be many
  - Node management screen
    - Start
    - Stop
    - Create New
    - Open Channel
    - Close Channel
    - Make Payment
  - An active list of successful / failed payments you are routing
  - Payments you have made
  - Searchable/paged list of nodes on the network
    - Selecting a node gives you actions you can make on it
      - Connecting
      - Opening channel
      - Exploit
  - A list of usable exploits to use
    - Selecting an exploit takes you to a page where you select the node to target.

- Exploits
  - Balance probe
  - Channel Jam
  - Probe for unannounced channels
  - Known CVE's on old versions of implementations

## Development

### Running

Copy the default config provided in this repo.

```
cp config.yaml local.config.yaml
```

Change the values if necessary and then run with those configs.

```
cargo run local.config.yaml
```

### Database Migrations

Changing any migration data will require running the migration locally on the development machine so that rust schema.rs code can also be generated:

```
DATABASE_URL=database.db diesel migration run
```

To create a new migration file, do this:

```
diesel migration generate node_channels
```

---

# Pages

## Actions

```
_Home View_
- Node Management
- Network View
- Routing
- Exploits
- Simulation Mode
```

---

## Node Management

```
_Node Management_
- [Back]
- [Create New]
- Node 1
- Node 2
```

### Node View

```
_Node 1_
- [Back]
- Connect To Peer
- List Channels
- Invoice
- Pay
- New On Chain Address
```

### Node Connect

```
_Enter node connect information:_
asdfasdfasdf@127.0.0.1:1937

> Success/Failure!
```

### List Channel

```
_Channel List:_
- [Back]
- [Create New]
- Channel 1
- Channel 2
```

### Open Channel

```
_Enter node open channel information:_
amount: 100000
node: asdfasdfasdfasdfasdfasdf
public or private: public

> Success/Failure!
- [Back]
```

### Channel Actions

```
_Channel Actions_
Status: Active
Balance: 5000
Capacity: 1000000

- [Back]
- Close
```

### Create Invoice

```
_Create Invoice_
amount: 5000
memo: hello

> lnc1.....

- [Back]
```

### Pay

```
_Pay_
invoice: lnc1....

> Success/Failure!

- [Back]
```

### New Address

```
_New Address_
> bc1.....

- [Back]
```

## Network View

```
_Network View_
- [Back]
- ACINQ (500 channels, 100 BTC Capacity)
- OpenNode (200 channels, 52 BTC Capacity)
```

```
_Node View_
ACINQ
Pubkey: 1ACB3...
Channels: 500
Capacity: 100 BTC

- [Back]
```

## Routing

```
_Routing_
Inbound      | Outbound        |  Amount
-----------------------------------------
Chan 1       | Chan 2          | 50000

- [Back]
```

## Exploits

```
_Exploits_
- [Back]
- Channel Jam
- Private Channel Probe
```

### Channel Jam

```
_Channel Jam_
- [Back]
- Active Jams
- New Jam
```

### Active Jams

```
_Active Jams_
- [Back]
- ACINQ Jam 1
- ACINQ Jam 2
- OpenNode Jam 1
```

### Jam View

```
_Jam View_
status; Full/Partial
channel: xyz

- [Back]
- Stop Jam
```

### New Jam

```
_New Jam_
target: 1AC...

> Which channel?
- Channel 1
- Channel 2
- Channel 3

> Success/Failure: "You need more channels", "You need more balance", etc..
```

### Private Channel Probe

```
_Private Channel Probe_
- [Back]
- Found Channels
- Active Campaigns
```

### Private Channel Probing Channels

```
_Found Private Channels_
- [Back]
- ACINQ : 0x011343AB : 5 BTC
- ACINQ : 0x302BCADC : 1 BTC
```

### Active Campaigns

```
_Active Campaigns_
- [Back]
- ACINQ
- OpenNode
```

### Capaign view

```
_Capaign View_
Status: 5000 / 10000000

- [Back]
- Stop Jamming
```

## Simulation Mode

```
_Simulation Mode_
- [Back]
- Configure Node A
- Configure Node B
- Configure Node C
- Generate to Address
- Start
- Points
```

### Generate to Address

```
_Configure Node X_
address: bc1...

> Success/Failure!
```

### Configure

```
_Configure Node X_
admin macaroon: asdfasdf...
tls cert: asdfasdf...

> Connected/Not Connected!
```

### Start

```
_Start Simulation_
> Started/Failed: "configure all nodes", etc.
```

```
_Points_
Total: 5
Simulation started: 1
Found Channels: 2
Jammed Channels: 1
Intercepted payment: 1

- [Back]
```

## User story for channel jamming

JAMMING ALICE->BOB->CAROL circle

user opens the app
they create a node
then look at that node:

- get on-chain address, copy that

go to simulation mode:

- generate to address (a couple times, so you're rich)
- mine some extra blocks to make sure you can spend

now they need to open two channels:

- peer with alice
- open channel with alice
- pay invoice to alice

- peer with carol
- open channel with carol
- pay invoice to carol

now set up ALICE->BOB->CAROL->ALICE circle (in polar, might already exist in the imported network)

create invoice on carol
pay invoice from alice

create invoice on alice
pay from carol

open chan jammer:

- pick the node (alice's node)
- pick the channel (alice's channel with carol)
- and click "jam"

### TO TEST

create invoice on carol
pay it from alice...

- if it worked, then you go to the routing view and see if you just routed a payment from alice to carol (because bob is jammed!)

### WOULD BE COOL

print out the route we create

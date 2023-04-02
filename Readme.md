### Logical Clock
Logical clock in distributed systems is mechanism to establish chronological and causal relationship between processes or events in distributed systems.

#### Lamport Clock
- [Lamport clock](https://lamport.azurewebsites.net/pubs/time-clocks.pdf) is a logical clock that helps us define causal order of events in a distrubited system. 
- This can be useful in determining partial order of events in a system (For example event A in node X "happened before" event B in node Y) 
- Algorithm of the lamport clock is simple and as following:

Sending a message:
```
time_stamp += 1;
send(message, time_stamp)
```

Receiving message:
```
(message, recv_time_stamp) = receive()
if (time_stamp < recv_time_stamp)
{
    time_stamp = recv_time_stamp;
    process(message)
}
else
{
    // probably received out-dated message.
}
time_stamp +=1
```
#### Vector Clock
Vector clock is a sophisticated variant of the logical clock which helps which gives us more gurantee, including concurrency and causal history. Each node holds a N-sized tuple, where N is the number of nodes in the distributed system and each index I holds a integer value representing timestamp at node I.
 
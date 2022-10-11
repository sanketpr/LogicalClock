### WIP

### Lamport Clock Based Chat Room
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

- Using this principle here we create a CLI chat room, where a node(or an instance) can send and receive a message using the above algorithm 
 
// imports
import 'package:web_socket_channel/io.dart';
import 'dart:io';

void main(List<String> args) {
  print("Enter address: ");
  String? addr = stdin.readLineSync();

  // creating a channel
  final channel = IOWebSocketChannel.connect(
    Uri.parse("ws://$addr"),
  );

  // send confirmation message
  channel.sink.add("Hello, Server!");

  // create a msg variable
  String? msg;

  // upon listening for a message
  channel.stream.listen((message) {
    // print the received message
    print("Received: $message");

    // prompt for new message to send
    print("Enter message: ");
    msg = stdin.readLineSync();
    channel.sink.add(msg);
  });
}

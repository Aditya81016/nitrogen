// imports
import 'package:web_socket_channel/io.dart';
import 'dart:io';

void main(List<String> args) {
  // creating a channel
  final channel = IOWebSocketChannel.connect(
    Uri.parse("ws://127.0.0.1:8080"),
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

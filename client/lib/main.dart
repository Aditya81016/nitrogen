import 'package:flutter/material.dart';
import 'package:nitrogen_client/templates/dr_driving.dart';
import 'package:web_socket_channel/io.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Nitrogen Client',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.indigo),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Nitrogen Client'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String serverStatus = "Not Connected";

  IOWebSocketChannel? channel;

  final serverIP = TextEditingController();
  final testCommand = TextEditingController();

  void createTemplate() {
    channel?.sink.add(testCommand.text);
  }

  void connect() {
    debugPrint("Disconnecting...");
    channel?.sink.close();
    setState(() {
      serverStatus = "Not Connected";
    });

    var addr = serverIP.text;
    debugPrint("Connecting to: $addr");
    channel = IOWebSocketChannel.connect('ws://$addr');
    debugPrint("Connected to: $addr");

    channel?.sink.add("Hello, Server!");
    channel?.stream.listen(
      (msg) {
        debugPrint("Received: $msg");
        setState(() {
          serverStatus = "Connected";
        });
      },
    );
  }

  @override
  void dispose() {
    // Clean up the controller when the widget is disposed.
    serverIP.dispose();
    testCommand.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Padding(
        padding: const EdgeInsets.all(15),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              "Server ($serverStatus)",
              textAlign: TextAlign.left,
              style: const TextStyle(fontSize: 25, fontWeight: FontWeight.w500),
            ),
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: serverIP,
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                      hintText: 'Server\'s ip address...',
                      contentPadding: EdgeInsets.fromLTRB(15, 0, 15, 0),
                    ),
                  ),
                ),
                ButtonTheme(
                  height: 10.5,
                  child: OutlinedButton(
                    onPressed: connect,
                    style: ButtonStyle(
                      shape: MaterialStateProperty.all(
                        RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(5.0),
                        ),
                      ),
                    ),
                    child: const Text("Connect"),
                  ),
                )
              ],
            ),
            const Padding(
              padding: EdgeInsets.fromLTRB(0, 15.0, 0, 5.0),
              child: Divider(),
            ),
            const Text(
              "Send Message",
              textAlign: TextAlign.left,
              style: TextStyle(fontSize: 25, fontWeight: FontWeight.w500),
            ),
            TextField(
              controller: testCommand,
              decoration: const InputDecoration(
                border: OutlineInputBorder(),
                hintText: 'Your message...',
                contentPadding: EdgeInsets.fromLTRB(15, 0, 15, 0),
              ),
            ),
            const Padding(
              padding: EdgeInsets.fromLTRB(0, 15.0, 0, 5.0),
              child: Divider(),
            ),
            const Text(
              "Templates",
              textAlign: TextAlign.left,
              style: TextStyle(fontSize: 25, fontWeight: FontWeight.w500),
            ),
            ElevatedButton(
                onPressed: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => DrDrivingTemplate(
                        passedChannel: channel,
                      ),
                    ),
                  );
                },
                child: const Text("Dr. Driving 2"))
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: createTemplate,
        tooltip: 'Send the command...',
        child: const Icon(Icons.send_rounded),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}

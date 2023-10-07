import 'dart:async';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:web_socket_channel/io.dart';
import 'package:sensors_plus/sensors_plus.dart';

class DrDrivingTemplate extends StatelessWidget {
  DrDrivingTemplate({super.key, IOWebSocketChannel? passedChannel}) {
    channel = passedChannel;
  }

  static IOWebSocketChannel? channel;

  static List<double> getPos(List<double> center, double radius, double theta) {
    List<double> pos = center;

    pos[0] += sin(theta * pi / 180) * radius;
    pos[1] += cos(theta * pi / 180) * radius;

    return pos;
  }

  static void handleMouse() async {
    AccelerometerEvent event = await accelerometerEvents.first;
    double roll = atan2(-event.y, -event.x) * 180 / pi;
    // double pitch =
    // atan2(-event.z, sqrt(pow(event.y, 2) + pow(event.x, 2))) * 180 / pi;
    // if (roll > 90) {
    //   roll -= 90;
    // }

    double x = (cos(roll * pi / 180) * 140) + 220;
    double y = (sin(roll * pi / 180) * 140) + 620;

    // debugPrint(roll.toString() + " | $x, $y");

    channel?.sink.add("mouse move $x $y");
  }

  @override
  Widget build(BuildContext context) {
    // channel?.sink.add("mouse press Left");
    Timer? timer;
    //     Timer.periodic(const Duration(milliseconds: 100), (timer) async {
    //   handleMouse();
    // });

    return Scaffold(
      // appBar: AppBar(
      //   title: const Text("Dr Driving Template"),
      // ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Button(
            pText: "Accelerate",
            pColor: Colors.green,
            pTarget: "Space",
          ),
          Expanded(
            flex: 3,
            child: Row(
              children: [
                Expanded(
                  child: Column(
                    children: [
                      Button(
                        pColor: Colors.deepOrange,
                        pText: "Engine",
                        pTarget: "Return",
                      ),
                      Button(
                        pColor: Colors.deepPurple,
                        pText: "Prima",
                        pTarget: "Alt",
                      ),
                      Button(
                        pColor: Colors.teal,
                        pText: "Escape",
                        pTarget: "Num5",
                      ),
                      Expanded(
                        child: ElevatedButton(
                          child: const Text("Handle Mouse"),
                          onPressed: () {
                            if (timer != null) {
                              channel?.sink.add("mouse release Left");
                              timer?.cancel();
                              timer = null;
                            } else {
                              channel?.sink.add("mouse press Left");
                              timer = Timer.periodic(
                                  Duration(milliseconds: (1000 / 30).round()),
                                  (timer) async {
                                handleMouse();
                              });
                            }
                          },
                        ),
                      ),
                      Expanded(
                        child: ElevatedButton(
                          child: const Text("Release"),
                          onPressed: () {
                            channel?.sink.add("release");
                          },
                        ),
                      ),
                    ],
                  ),
                ),
                Expanded(
                  child: Column(
                    children: [
                      Button(
                        pColor: Colors.amber,
                        pText: "Park",
                        pTarget: "Num1",
                      ),
                      Button(
                        pColor: Colors.cyan,
                        pText: "Reverse",
                        pTarget: "Num2",
                      ),
                      Button(
                        pColor: Colors.indigo,
                        pText: "Neutral",
                        pTarget: "Num3",
                      ),
                      Button(
                        pColor: Colors.lime,
                        pText: "Driving",
                        pTarget: "Num4",
                      )
                    ],
                  ),
                ),
              ],
            ),
          ),
          Button(
            pText: "Brake",
            pColor: Colors.red,
            pTarget: "ControlLeft",
          )
        ],
      ),
    );
  } //
}

class Button extends StatelessWidget {
  Button({super.key, Color? pColor, String? pTarget, String? pText}) {
    text = pText ?? "Button";
    color = pColor ?? Colors.amber[900] as Color;
    target = pTarget ?? "Space";
  }

  late String text;
  late Color color;
  late String target;
  IOWebSocketChannel? channel = DrDrivingTemplate.channel;
  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: InkResponse(
          onTapDown: (dt) {
            channel?.sink.add("keyboard press $target");
          },
          onTapUp: (dt) {
            channel?.sink.add("keyboard release $target");
          },
          child: Container(
            color: color,
            child: Center(
              child: Text(
                text,
                style: const TextStyle(
                    fontWeight: FontWeight.w600, color: Colors.white),
              ),
            ),
          )),
    );
  }
}
//

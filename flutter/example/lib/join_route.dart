import 'package:flutter/material.dart';

import 'call_route.dart';
import 'package:faker_dart/faker_dart.dart';

class JoinRoute extends StatefulWidget {
  const JoinRoute({super.key});

  @override
  State<JoinRoute> createState() => _JoinRouteState();
}

final _faker = Faker.instance;
const defaultRoomId = 'pub-pub-video-call';
final defaultMemberId = _faker.name.firstName();

class _JoinRouteState extends State<JoinRoute> {
  String _roomId = defaultRoomId;
  String _memberId = defaultMemberId;

  bool isPublish = true;
  bool publishAudio = true;
  bool publishVideo = true;
  bool fakeMedia = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Jason demo')),
      body: Center(
        child: ListView(
          padding: const EdgeInsets.all(20),
          children: [
            Image.asset('assets/images/logo.png', height: 200),
            TextFormField(
              initialValue: defaultRoomId,
              onChanged: (text) => setState(() => _roomId = text),
              decoration: const InputDecoration(hintText: 'Room ID'),
            ),
            TextFormField(
              initialValue: defaultMemberId,
              onChanged: (text) => setState(() => _memberId = text),
              decoration: const InputDecoration(hintText: 'Member ID'),
            ),
            SwitchListTile(
              title: const Text('Publish'),
              value: isPublish,
              onChanged: (v) => setState(() => isPublish = v),
            ),
            SwitchListTile(
              title: const Text('Publish Video'),
              value: publishVideo,
              onChanged: (v) => setState(() => publishVideo = v),
            ),
            SwitchListTile(
              title: const Text('Publish Audio'),
              value: publishAudio,
              onChanged: (v) => setState(() => publishAudio = v),
            ),
            SwitchListTile(
              title: const Text('FakeMedia'),
              value: fakeMedia,
              onChanged: (v) => setState(() => fakeMedia = v),
            ),
            TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
                disabledForegroundColor: Colors.grey.withValues(0.38),
              ),
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => CallRoute(
                      _roomId,
                      _memberId,
                      isPublish,
                      publishVideo,
                      publishAudio,
                      fakeMedia,
                    ),
                  ),
                );
              },
              child: const Text('Join Room'),
            )
          ],
        ),
      ),
    );
  }
}

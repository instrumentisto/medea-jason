import 'package:flutter/material.dart';

import 'call_route.dart';

class JoinRoute extends StatefulWidget {
  @override
  _JoinRouteState createState() => _JoinRouteState();
}

const DEFAULT_ROOM_ID = 'pub-pub-video-call';
const DEFAULT_MEMBER_ID = 'caller';

class _JoinRouteState extends State<JoinRoute> {
  String _roomId = DEFAULT_ROOM_ID;
  String _memberId = DEFAULT_MEMBER_ID;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Jason demo'),
      ),
      body: Center(
          child: Container(
              padding: EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Image.asset('assets/images/logo.png', height: 200),
                  TextFormField(
                    initialValue: DEFAULT_ROOM_ID,
                    onChanged: (text) {
                      setState(() {
                        _roomId = text;
                      });
                    },
                    decoration: InputDecoration(
                      hintText: 'Room ID',
                    ),
                  ),
                  TextFormField(
                    initialValue: DEFAULT_MEMBER_ID,
                    onChanged: (text) {
                      setState(() {
                        _memberId = text;
                      });
                    },
                    decoration: InputDecoration(
                      hintText: 'Member ID',
                    ),
                  ),
                  TextButton(
                    style: TextButton.styleFrom(
                      foregroundColor: Colors.white,
                      backgroundColor: Colors.blue,
                      disabledForegroundColor: Colors.grey.withOpacity(0.38),
                    ),
                    onPressed: () {
                      print('RoomID: $_roomId and MemberID: $_memberId');
                      Navigator.push(
                          context,
                          MaterialPageRoute(
                              builder: (context) =>
                                  CallRoute(_roomId, _memberId)));
                    },
                    child: Text('Join Room'),
                  )
                ],
              ))),
    );
  }
}

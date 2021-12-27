import 'package:flutter/material.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';

import 'call.dart';

class CallRoute extends StatefulWidget {
  final String _roomId;
  final String _memberId;

  CallRoute(this._roomId, this._memberId);

  @override
  _CallState createState() => _CallState(_roomId, _memberId);
}

class _CallState extends State {
  bool _videoEnabled = true;
  bool _audioEnabled = true;
  final List<RTCVideoView> _videos = List.empty(growable: true);
  final Call _call = Call();
  late String _roomId;
  late String _memberId;

  _CallState(String roomId, String memberId) {
    _roomId = roomId;
    _memberId = memberId;
  }

  @override
  void initState() {
    _call.onNewRemoteStream((stream) async {
      var renderer = RTCVideoRenderer();
      await renderer.initialize();
      renderer.srcObject = stream;
      setState(() {
        _videos.add(RTCVideoView(renderer));
      });
    });
    _call.onLocalStream((stream) async {
      var renderer = RTCVideoRenderer();
      await renderer.initialize();
      renderer.srcObject = stream;
      setState(() {
        _videos.add(RTCVideoView(renderer, mirror: true));
      });
    });
    _call.start(_roomId, _memberId);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: Text('Medea call demo'),
        ),
        body: Center(
            child: Container(
                width: MediaQuery.of(context).size.width,
                height: MediaQuery.of(context).size.height,
                child: Column(
                  children:
                      _videos.map((video) => Expanded(child: video)).toList(),
                ))),
        floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
        floatingActionButton: Padding(
            padding: EdgeInsets.only(bottom: 50.0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Padding(
                    padding: EdgeInsets.only(right: 30.0),
                    child: FloatingActionButton(
                      onPressed: () async {
                        setState(() {
                          _audioEnabled = !_audioEnabled;
                        });
                        await _call.toggleAudio(_audioEnabled);
                      },
                      heroTag: null,
                      child: Icon(_audioEnabled ? Icons.mic_off : Icons.mic),
                    )),
                Padding(
                    padding: EdgeInsets.only(right: 30.0),
                    child: FloatingActionButton(
                      onPressed: () async {
                        setState(() {
                          _videoEnabled = !_videoEnabled;
                        });
                        await _call.toggleVideo(_videoEnabled);
                      },
                      heroTag: null,
                      child: Icon(
                          _videoEnabled ? Icons.videocam_off : Icons.videocam),
                    )),
                FloatingActionButton(
                  onPressed: () async {
                    _call.dispose();
                  },
                  heroTag: null,
                  backgroundColor: Colors.red,
                  child: Icon(Icons.call_end),
                ),
              ],
            )));
  }
}

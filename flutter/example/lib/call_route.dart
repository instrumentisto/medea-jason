import 'package:flutter/material.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'call.dart';

class CallRoute extends StatefulWidget {
  final String _roomId;
  final String _memberId;
  final bool _isPublish;
  final bool _publishAudio;
  final bool _publishVideo;
  final bool _fakeMedia;

  CallRoute(this._roomId, this._memberId, this._isPublish, this._publishVideo,
      this._publishAudio, this._fakeMedia);

  @override
  _CallState createState() => _CallState(
      _roomId, _memberId, _isPublish, _publishVideo, _publishAudio, _fakeMedia);
}

class _CallState extends State {
  late bool _isPublish;
  late bool _publishAudio;
  late bool _publishVideo;
  late bool _fakeMedia;

  bool _screenShare = false;

  bool _videoEnabled = true;
  bool _audioEnabled = true;

  bool _videoSend = true;
  bool _videoRecv = true;
  bool _audioSend = true;
  bool _audioRecv = true;

  final List<VideoView> _videos = List.empty(growable: true);
  final Call _call = Call();
  late String _roomId;
  late String _memberId;

  _CallState(String roomId, String memberId, bool isPublish, bool publishVideo,
      bool publishAudio, bool fakeMedia) {
    _roomId = roomId;
    _memberId = memberId;
    _isPublish = isPublish;
    _publishVideo = publishVideo;
    _publishAudio = publishAudio;
    _fakeMedia = fakeMedia;
  }

  @override
  void initState() {
    _call.onNewRemoteStream((track) async {
      var renderer = createVideoRenderer();
      await renderer.initialize();
      await renderer.setSrcObject(track);
      setState(() {
        _videos.add(VideoView(renderer));
      });
    });
    _call.onLocalStream((track) async {
      var renderer = createVideoRenderer();
      await renderer.initialize();
      await renderer.setSrcObject(track);
      setState(() {
        _videos.add(VideoView(renderer, mirror: true));
      });
    });
    _call.start(_roomId, _memberId, _isPublish, _publishVideo, _publishAudio,
        _fakeMedia);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(title: Text('Medea call demo'), actions: <Widget>[
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Device'),
              onPressed: () async {
                setState(() {});

                await showDialog<void>(
                  context: context,
                  builder: (BuildContext context) {
                    return AlertDialog(
                      content: StatefulBuilder(
                        builder:
                            (BuildContext context, StateSetter setStateSb) {
                          return Column(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              SwitchListTile(
                                  title: Text('Screen share'),
                                  value: _screenShare,
                                  onChanged: (v) => setStateSb(() {
                                        _screenShare = v;
                                      })),
                            ],
                          );
                        },
                      ),
                    );
                  },
                );

                await _call.toggleScreenShare(_screenShare);
              }),
        ]),
        body: Center(
            child: Container(
                width: MediaQuery.of(context).size.width,
                height: MediaQuery.of(context).size.height,
                child: Column(
                  children:
                      _videos.map((video) => Expanded(child: video)).toList(),
                ))),
        floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
        persistentFooterButtons: [
          TextButton(
              onPressed: () async {
                await _call.setSendAudio(!_audioSend);
                setState(() {
                  _audioSend = !_audioSend;
                });
              },
              child: Text((_audioSend ? 'Disable' : 'Enable') + ' Audio Send')),
          TextButton(
              onPressed: () async {
                await _call.setRecvAudio(!_audioSend);
                setState(() {
                  _audioRecv = !_audioRecv;
                });
              },
              child: Text((_audioRecv ? 'Disable' : 'Enable') + ' Audio Recv')),
          TextButton(
              onPressed: () async {
                await _call.setSendVideo(!_videoSend);
                setState(() {
                  _videoSend = !_videoSend;
                });
              },
              child: Text((_videoSend ? 'Disable' : 'Enable') + ' Video Send')),
          TextButton(
              onPressed: () async {
                await _call.setRecvVideo(!_videoRecv);
                setState(() {
                  _videoRecv = !_videoRecv;
                });
              },
              child: Text((_videoRecv ? 'Disable' : 'Enable') + ' Video Recv')),
        ],
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
                    await _call.dispose();
                    Navigator.pop(context);
                  },
                  heroTag: null,
                  backgroundColor: Colors.red,
                  child: Icon(Icons.call_end),
                ),
              ],
            )));
  }
}

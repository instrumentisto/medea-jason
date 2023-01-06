import 'package:flutter/material.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';
import 'package:medea_jason/medea_jason.dart' as jason;

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

  String? display;
  String? videoDevice;
  String? audioDevice;

  bool _screenShare = false;

  bool _videoEnabled = true;
  bool _audioEnabled = true;

  bool _videoSend = true;
  bool _videoRecv = true;
  bool _audioSend = true;
  bool _audioRecv = true;

  VideoView? displayView;
  VideoView? deviceView;

  final Map<String, List<VideoView>> _videos = {};
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
    _call.onNewRemoteStream((track, remoteId) async {
      var renderer = createVideoRenderer();
      await renderer.initialize();
      await renderer.setSrcObject(track);
      var remoteTracks = _videos[remoteId];
      if (remoteTracks == null) {
        remoteTracks = List.empty(growable: true);
        remoteTracks.add(VideoView(renderer, mirror: true));
      } else {
        remoteTracks.add(VideoView(renderer, mirror: true));
      }
      setState(() {
        _videos[remoteId] = remoteTracks!;
      });
    });
    _call.onLocalDeviceStream((track) async {
      if (deviceView == null) {
        var renderer = createVideoRenderer();
        await renderer.initialize();
        await renderer.setSrcObject(track);
        var localTracks = _videos['I'];
        deviceView = VideoView(renderer, mirror: true);
        if (localTracks == null) {
          localTracks = List.empty(growable: true);
          localTracks.add(deviceView!);
        } else {
          localTracks.add(deviceView!);
        }
        setState(() {
          _videos['I'] = localTracks!;
        });
      } else {
        await deviceView!.videoRenderer.setSrcObject(track);
      }
    });
    _call.onLocalDisplayStream((track) async {
      if (displayView == null) {
        var renderer = createVideoRenderer();
        await renderer.initialize();
        await renderer.setSrcObject(track);
        var localTracks = _videos['I'];
        displayView = VideoView(renderer, mirror: true);
        if (localTracks == null) {
          localTracks = List.empty(growable: true);
          localTracks.add(displayView!);
        } else {
          localTracks.add(displayView!);
        }
        setState(() {
          _videos['I'] = localTracks!;
        });
      } else {
        await displayView!.videoRenderer.setSrcObject(track);
      }
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
              child: Text('Display'),
              onPressed: () async {
                setState(() {});
                var displayList = await _call.enumerateDisplay();
                var d = jason.DisplayVideoTrackConstraints();
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
                              DropdownButton<String>(
                                value: display,
                                icon: const Icon(Icons.arrow_downward),
                                elevation: 16,
                                style:
                                    const TextStyle(color: Colors.deepPurple),
                                underline: Container(
                                  height: 2,
                                  color: Colors.deepPurpleAccent,
                                ),
                                onChanged: (String? value) {
                                  // This is called when the user selects an item.
                                  setStateSb(() {
                                    display = value;
                                  });
                                },
                                items: displayList
                                    .map<DropdownMenuItem<String>>((value) {
                                  return DropdownMenuItem<String>(
                                    value: value.title() == null
                                        ? value.deviceId()
                                        : value.title()!,
                                    child: Text(value.title() == null
                                        ? value.deviceId()
                                        : value.title()!),
                                  );
                                }).toList(),
                              ),
                              TextFormField(
                                initialValue: '30',
                                keyboardType: TextInputType.number,
                                onChanged: (text) {
                                  try {
                                    d.idealFrameRate(int.parse(text));
                                    // ignore: empty_catches
                                  } catch (e) {}
                                },
                                decoration: InputDecoration(
                                  labelText: 'Display FPS',
                                ),
                              ),
                              TextFormField(
                                initialValue: '640',
                                keyboardType: TextInputType.number,
                                onChanged: (text) {
                                  try {
                                    d.idealWidth(int.parse(text));
                                    // ignore: empty_catches
                                  } catch (e) {}
                                },
                                decoration: InputDecoration(
                                  labelText: 'Display width',
                                ),
                              ),
                              TextFormField(
                                initialValue: '480',
                                keyboardType: TextInputType.number,
                                onChanged: (text) {
                                  try {
                                    d.idealHeight(int.parse(text));
                                    // ignore: empty_catches
                                  } catch (e) {}
                                },
                                decoration: InputDecoration(
                                  labelText: 'Display height',
                                ),
                              ),
                            ],
                          );
                        },
                      ),
                    );
                  },
                );
                await _call.toggleScreenShare(_screenShare, d);
              }),

          TextButton(
                          style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('SendRecv'),
              onPressed: () async {
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
                              TextButton(
                                  onPressed: () async {
                                    await _call.setSendAudio(!_audioSend);
                                    setStateSb(() {
                                      _audioSend = !_audioSend;
                                    });
                                  },
                                  child: Text(
                                      (_audioSend ? 'Disable' : 'Enable') +
                                          ' Audio Send')),
                              TextButton(
                                  onPressed: () async {
                                    await _call.setRecvAudio(!_audioSend);
                                    setStateSb(() {
                                      _audioRecv = !_audioRecv;
                                    });
                                  },
                                  child: Text(
                                      (_audioRecv ? 'Disable' : 'Enable') +
                                          ' Audio Recv')),
                              TextButton(
                                  onPressed: () async {
                                    await _call.setSendVideo(!_videoSend);
                                    setStateSb(() {
                                      _videoSend = !_videoSend;
                                    });
                                  },
                                  child: Text(
                                      (_videoSend ? 'Disable' : 'Enable') +
                                          ' Video Send')),
                              TextButton(
                                  onPressed: () async {
                                    await _call.setRecvVideo(!_videoRecv);
                                    setStateSb(() {
                                      _videoRecv = !_videoRecv;
                                    });
                                  },
                                  child: Text(
                                      (_videoRecv ? 'Disable' : 'Enable') +
                                          ' Video Recv')),
                            ],
                          );
                        },
                      ),
                    );
                  },
                );
              }),
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Device'),
              onPressed: () async {
                setState(() {});
                var deviceList = await _call.enumerateDevice();
                var videoDevices = deviceList
                    .where((element) =>
                        element.kind() == jason.MediaDeviceKind.videoinput)
                    .toList();
                var audioDevices = deviceList
                    .where((element) =>
                        element.kind() == jason.MediaDeviceKind.audioinput)
                    .toList();
                var d = jason.DeviceVideoTrackConstraints();
                var vd = jason.AudioTrackConstraints();
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
                              DropdownButton<String>(
                                value: videoDevice,
                                icon: const Icon(Icons.arrow_downward),
                                elevation: 16,
                                style:
                                    const TextStyle(color: Colors.deepPurple),
                                underline: Container(
                                  height: 2,
                                  color: Colors.deepPurpleAccent,
                                ),
                                onChanged: (String? value) {
                                  // This is called when the user selects an item.
                                  setStateSb(() {
                                    videoDevice = value;
                                  });
                                },
                                items: videoDevices
                                    .map<DropdownMenuItem<String>>((value) {
                                  return DropdownMenuItem<String>(
                                    value: value.label(),
                                    child: Text(value.label()),
                                  );
                                }).toList(),
                              ),
                              DropdownButton<String>(
                                value: audioDevice,
                                icon: const Icon(Icons.arrow_downward),
                                elevation: 16,
                                style:
                                    const TextStyle(color: Colors.deepPurple),
                                underline: Container(
                                  height: 2,
                                  color: Colors.deepPurpleAccent,
                                ),
                                onChanged: (String? value) {
                                  // This is called when the user selects an item.
                                  setStateSb(() {
                                    audioDevice = value;
                                    vd.deviceId(value!);
                                  });
                                },
                                items: audioDevices
                                    .map<DropdownMenuItem<String>>((value) {
                                  return DropdownMenuItem<String>(
                                    value: value.label(),
                                    child: Text(value.label()),
                                  );
                                }).toList(),
                              ),
                              TextFormField(
                                initialValue: '640',
                                keyboardType: TextInputType.number,
                                onChanged: (text) {
                                  try {
                                    d.idealWidth(int.parse(text));
                                    // ignore: empty_catches
                                  } catch (e) {}
                                },
                                decoration: InputDecoration(
                                  labelText: 'Device width',
                                ),
                              ),
                              TextFormField(
                                initialValue: '480',
                                keyboardType: TextInputType.number,
                                onChanged: (text) {
                                  try {
                                    d.idealHeight(int.parse(text));
                                    // ignore: empty_catches
                                  } catch (e) {}
                                },
                                decoration: InputDecoration(
                                  labelText: 'Device height',
                                ),
                              ),
                            ],
                          );
                        },
                      ),
                    );
                  },
                );

                await _call.setVideoDevices(d);
                // await _call.setAudioDevices(vd);
              }),
        ]),
        body: Center(
            child: Container(
                width: MediaQuery.of(context).size.width,
                height: MediaQuery.of(context).size.height,
                child: Row(
                  children: _videos.values
                      .map((videoList) => Expanded(
                          child: Column(
                              children: videoList
                                  .map((video) => Expanded(child: video))
                                  .toList())))
                      .toList(),
                ))),
        floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
        // bottomNavigationBar: ListView(
        //   scrollDirection: Axis.horizontal,
        //   children: [
        //     TextButton(
        //       onPressed: () async {
        //         await _call.setSendAudio(!_audioSend);
        //         setState(() {
        //           _audioSend = !_audioSend;
        //         });
        //       },
        //       child: Text((_audioSend ? 'Disable' : 'Enable') + ' Audio Send')),
        //   TextButton(
        //       onPressed: () async {
        //         await _call.setSendAudio(!_audioSend);
        //         setState(() {
        //           _audioSend = !_audioSend;
        //         });
        //       },
        //       child: Text((_audioSend ? 'Disable' : 'Enable') + ' Audio Send')),
        //   TextButton(
        //       onPressed: () async {
        //         await _call.setRecvAudio(!_audioSend);
        //         setState(() {
        //           _audioRecv = !_audioRecv;
        //         });
        //       },
        //       child: Text((_audioRecv ? 'Disable' : 'Enable') + ' Audio Recv')),
        //   TextButton(
        //       onPressed: () async {
        //         await _call.setSendVideo(!_videoSend);
        //         setState(() {
        //           _videoSend = !_videoSend;
        //         });
        //       },
        //       child: Text((_videoSend ? 'Disable' : 'Enable') + ' Video Send')),
        //   TextButton(
        //       onPressed: () async {
        //         await _call.setRecvVideo(!_videoRecv);
        //         setState(() {
        //           _videoRecv = !_videoRecv;
        //         });
        //       },
        //       child: Text((_videoRecv ? 'Disable' : 'Enable') + ' Video Recv')),
        // ]),
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

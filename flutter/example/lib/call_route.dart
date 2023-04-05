import 'package:flutter/material.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';
import 'package:medea_jason/medea_jason.dart' as jason;
import 'package:medea_jason_example/control_api/entities/endpoint.dart';
import 'package:medea_jason_example/control_api/entities/member.dart';

import 'call.dart';

bool _videoSend = true;
bool _videoRecv = true;
bool _audioSend = true;
bool _audioRecv = true;

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

  bool _videoEnabled = true;
  bool _audioEnabled = true;

  VideoView? localScreenVideo;
  VideoView? localCameraVideo;

  final Map<String, Map<String, VideoView>> _videos = {};
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
      var trackId = track.getTrack().id();
      if (track.mediaDirection() == jason.TrackMediaDirection.SendRecv) {
        var renderer = createVideoRenderer();
        await renderer.initialize();
        await renderer.setSrcObject(track.getTrack());

        var remoteTracks = _videos[remoteId];
        if (remoteTracks == null) {
          remoteTracks = Map.from({trackId: VideoView(renderer, mirror: true)});
        } else {
          remoteTracks[trackId] = VideoView(renderer, mirror: true);
        }

        setState(() {
          _videos[remoteId] = remoteTracks!;
        });
      }

      track.onMediaDirectionChanged(
        (new_dir) async {
          var remoteTracks = _videos[remoteId];

          if (new_dir == jason.TrackMediaDirection.SendRecv) {
            var renderer = createVideoRenderer();
            await renderer.initialize();
            await renderer.setSrcObject(track.getTrack());

            if (remoteTracks == null) {
              remoteTracks =
                  Map.from({trackId: VideoView(renderer, mirror: true)});
            } else {
              remoteTracks[trackId] = VideoView(renderer, mirror: true);
            }
          } else {
            if (remoteTracks != null) {
              remoteTracks.remove(trackId);
            }
          }

          setState(() {
            _videos[remoteId] = remoteTracks!;
          });
        },
      );
    });

    _call.onLocalDeviceStream((track) async {
      if (localCameraVideo == null) {
        var renderer = createVideoRenderer();
        await renderer.initialize();
        await renderer.setSrcObject(track);
        localCameraVideo = VideoView(renderer, mirror: true);

        var localTracks = _videos['I'];
        if (localTracks == null) {
          localTracks = Map.from({'I': localCameraVideo!});
        } else {
          localTracks['I'] = localCameraVideo!;
        }
        setState(() {
          _videos['I'] = localTracks!;
        });
      } else {
        await localCameraVideo!.videoRenderer.setSrcObject(track);
      }
    });

    _call.onLocalDisplayStream((track) async {
      if (localScreenVideo == null) {
        var renderer = createVideoRenderer();
        await renderer.initialize();
        await renderer.setSrcObject(track);
        localScreenVideo = VideoView(renderer, mirror: true);

        var localTracks = _videos['I'];
        if (localTracks == null) {
          localTracks = Map.from({'I': localScreenVideo!});
        } else {
          localTracks['I'] = localScreenVideo!;
        }
        setState(() {
          _videos['I'] = localTracks!;
        });
      } else {
        await localScreenVideo!.videoRenderer.setSrcObject(track);
      }
    });

    _call.onDeviceChange(() {
      var snackBar = SnackBar(content: Text('On device change'));
      ScaffoldMessenger.of(context).showSnackBar(snackBar);
    });

    _call.onError((err) {
      var snackBar = SnackBar(content: Text(err));
      ScaffoldMessenger.of(context).showSnackBar(snackBar);
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
              child: Text('MediaSetting'),
              onPressed: () async {
                await mediaSettingDialog(context, _call);
              }),
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Create'),
              onPressed: () async {
                await controlApiCreateDialog(context, _call);
              }),
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Get'),
              onPressed: () async {
                await controlApiGetDialog(context, _call);
              }),
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Delete'),
              onPressed: () async {
                await controlApiDeleteDialog(context, _call);
              }),
          TextButton(
              style: TextButton.styleFrom(
                foregroundColor: Colors.white,
                backgroundColor: Colors.blue,
              ),
              child: Text('Callbacks'),
              onPressed: () async {
                await showCallbacks(context, _call);
              }),
        ]),
        body: Center(
            child: Container(
                width: MediaQuery.of(context).size.width,
                height: MediaQuery.of(context).size.height,
                child: Row(
                  children: _videos.values
                      .map((videoMap) => Expanded(
                          child: Column(
                              children: videoMap.values
                                  .map((video) => Expanded(child: video))
                                  .toList())))
                      .toList(),
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

Future showCallbacks(BuildContext context, Call call) async {
  var cbs = await call.controlApi.getCallbacks();
  await showDialog<void>(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
            content: Container(
                width: double.maxFinite,
                child: ListView(
                  shrinkWrap: true,
                  children: cbs
                      .map((cb) => Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Expanded(
                                  child: Text(cb.event.toJson().toString())),
                              Expanded(child: Text(cb.at)),
                              Expanded(child: Text(cb.fid)),
                            ],
                          ))
                      .toList(),
                )));
      });
}

Future controlApiGetDialog(BuildContext context, Call call) async {
  var roomId = '';
  var memberId = '';
  var endpointId = '';

  await showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Flexible(
                      child: Text('local://'),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newRoomId) async {
                          roomId = newRoomId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Room ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newMemberId) async {
                          memberId = newMemberId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Member ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newEndpointId) async {
                          endpointId = newEndpointId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Endpoint ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                  ],
                ),
                SizedBox(height: 10),
                TextButton(
                    onPressed: () async {
                      var resp = await call.controlApi
                          .get(roomId, memberId, endpointId);

                      print(resp);

                      Navigator.pop(context);
                    },
                    child: Text('Get'))
              ],
            );
          },
        ),
      );
    },
  );
}

Future controlApiDeleteDialog(BuildContext context, Call call) async {
  var roomId = '';
  var memberId = '';
  var endpointId = '';

  await showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Flexible(
                      child: Text('local://'),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newRoomId) async {
                          roomId = newRoomId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Room ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newMemberId) async {
                          memberId = newMemberId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Member ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newEndpointId) async {
                          endpointId = newEndpointId;
                        },
                        decoration: InputDecoration(
                          hintText: 'Endpoint ID',
                        ),
                      ),
                    ),
                    SizedBox(width: 10),
                  ],
                ),
                SizedBox(height: 10),
                TextButton(
                    onPressed: () async {
                      await call.controlApi
                          .delete(roomId, memberId, endpointId);
                      Navigator.pop(context);
                    },
                    child: Text('Get'))
              ],
            );
          },
        ),
      );
    },
  );
}

Future mediaSettingDialog(BuildContext context, Call call) async {
  var displayList = await call.enumerateDisplay();

  var deviceList = await call.enumerateDevice();
  var videoDevices = deviceList
      .where((element) => element.kind() == jason.MediaDeviceKind.VideoInput)
      .toList();
  var audioDevices = deviceList
      .where((element) => element.kind() == jason.MediaDeviceKind.AudioInput)
      .toList();

  await showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                SwitchListTile(
                    title: Text('Screen share'),
                    value: call.screenShare,
                    onChanged: (v) => setStateSb(() {
                          call.screenShare = v;
                        })),
                DropdownButton<String>(
                  value: call.videoDisplayId,
                  icon: const Icon(Icons.arrow_downward),
                  elevation: 16,
                  style: const TextStyle(color: Colors.deepPurple),
                  underline: Container(
                    height: 2,
                    color: Colors.deepPurpleAccent,
                  ),
                  onChanged: (String? value) {
                    // This is called when the user selects an item.
                    setStateSb(() {
                      call.videoDisplayId = value!;
                    });
                  },
                  items: displayList.map<DropdownMenuItem<String>>((value) {
                    return DropdownMenuItem<String>(
                      value: value.deviceId(),
                      child: Text(value.title() == null
                          ? value.deviceId()
                          : value.title()!),
                    );
                  }).toList(),
                ),
                TextFormField(
                  initialValue: call.selectedDisplayFrameRate == null
                      ? '30'
                      : call.selectedDisplayFrameRate.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDisplayFrameRate = int.parse(text);
                      // ignore: empty_catches
                    } catch (e) {}
                  },
                  decoration: InputDecoration(
                    labelText: 'Display FPS',
                  ),
                ),
                TextFormField(
                  initialValue: call.selectedDisplayWidth == null
                      ? '640'
                      : call.selectedDisplayWidth.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDisplayWidth = int.parse(text);
                      // ignore: empty_catches
                    } catch (e) {}
                  },
                  decoration: InputDecoration(
                    labelText: 'Display width',
                  ),
                ),
                TextFormField(
                  initialValue: call.selectedDisplayHeight == null
                      ? '480'
                      : call.selectedDisplayHeight.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDisplayHeight = int.parse(text);
                      // ignore: empty_catches
                    } catch (e) {}
                  },
                  decoration: InputDecoration(
                    labelText: 'Display height',
                  ),
                ),
                DropdownButton<String>(
                  value: call.videoDeviceId,
                  icon: const Icon(Icons.arrow_downward),
                  elevation: 16,
                  style: const TextStyle(color: Colors.deepPurple),
                  underline: Container(
                    height: 2,
                    color: Colors.deepPurpleAccent,
                  ),
                  onChanged: (String? value) {
                    // This is called when the user selects an item.
                    setStateSb(() {
                      call.videoDeviceId = value;
                    });
                  },
                  items: videoDevices.map<DropdownMenuItem<String>>((value) {
                    return DropdownMenuItem<String>(
                      value: value.deviceId(),
                      child: Text(value.label()),
                    );
                  }).toList(),
                ),
                DropdownButton<String>(
                  value: call.audioDeviceId,
                  icon: const Icon(Icons.arrow_downward),
                  elevation: 16,
                  style: const TextStyle(color: Colors.deepPurple),
                  underline: Container(
                    height: 2,
                    color: Colors.deepPurpleAccent,
                  ),
                  onChanged: (String? value) {
                    // This is called when the user selects an item.
                    setStateSb(() {
                      call.audioDeviceId = value;
                    });
                  },
                  items: audioDevices.map<DropdownMenuItem<String>>((value) {
                    return DropdownMenuItem<String>(
                      value: value.deviceId(),
                      child: Text(value.label()),
                    );
                  }).toList(),
                ),
                TextFormField(
                  initialValue: call.selectedDeviceWidth == null
                      ? '640'
                      : call.selectedDeviceWidth.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDeviceWidth = int.parse(text);
                      // ignore: empty_catches
                    } catch (e) {}
                  },
                  decoration: InputDecoration(
                    labelText: 'Device width',
                  ),
                ),
                TextFormField(
                  initialValue: call.selectedDeviceHeight == null
                      ? '480'
                      : call.selectedDeviceHeight.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDeviceHeight = int.parse(text);
                      // ignore: empty_catches
                    } catch (e) {}
                  },
                  decoration: InputDecoration(
                    labelText: 'Device height',
                  ),
                ),
                Row(
                  children: [
                    TextButton(
                        onPressed: () async {
                          await call.setSendAudio(!_audioSend);
                          setStateSb(() {
                            _audioSend = !_audioSend;
                          });
                        },
                        child: Text((_audioSend ? 'Disable' : 'Enable') +
                            ' Audio Send')),
                    TextButton(
                        onPressed: () async {
                          await call.setSendVideo(!_videoSend);
                          setStateSb(() {
                            _videoSend = !_videoSend;
                          });
                        },
                        child: Text((_videoSend ? 'Disable' : 'Enable') +
                            ' Video Send')),
                  ],
                ),
                Row(
                  children: [
                    TextButton(
                        onPressed: () async {
                          await call.setRecvAudio(!_audioRecv);
                          setStateSb(() {
                            _audioRecv = !_audioRecv;
                          });
                        },
                        child: Text((_audioRecv ? 'Disable' : 'Enable') +
                            ' Audio Recv')),
                    TextButton(
                        onPressed: () async {
                          await call.setRecvVideo(!_videoRecv);
                          setStateSb(() {
                            _videoRecv = !_videoRecv;
                          });
                        },
                        child: Text((_videoRecv ? 'Disable' : 'Enable') +
                            ' Video Recv')),
                  ],
                ),
                SizedBox(
                  height: 10,
                ),
                TextButton(
                    onPressed: () async {
                      var videoTrack = jason.DeviceVideoTrackConstraints();
                      videoTrack.deviceId(call.videoDeviceId!);
                      if (call.selectedDeviceHeight != null) {
                        videoTrack.exactHeight(call.selectedDeviceHeight!);
                      }
                      if (call.selectedDeviceWidth != null) {
                        videoTrack.exactWidth(call.selectedDeviceWidth!);
                      }

                      var displayTrack = jason.DisplayVideoTrackConstraints();
                      if (call.videoDisplayId != null) {
                        displayTrack.deviceId(call.videoDisplayId!);
                      }
                      if (call.selectedDisplayHeight != null) {
                        displayTrack.exactHeight(call.selectedDisplayHeight!);
                      }
                      if (call.selectedDisplayWidth != null) {
                        displayTrack.exactWidth(call.selectedDisplayWidth!);
                      }
                      if (call.selectedDisplayFrameRate != null) {
                        displayTrack
                            .exactFrameRate(call.selectedDisplayFrameRate!);
                      }

                      var audioTrack = jason.AudioTrackConstraints();
                      audioTrack.deviceId(call.audioDeviceId!);

                      await call.setMedia(videoTrack, audioTrack, displayTrack);
                    },
                    child: Text('Set media setting')),
              ],
            );
          },
        ),
      );
    },
  );
}

Future controlApiCreateDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(mainAxisSize: MainAxisSize.min, children: [
              TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: Text('Room'),
                  onPressed: () async {
                    await controlApiCreateRoomDialog(context, call);
                  }),
              SizedBox(height: 10),
              TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: Text('Member'),
                  onPressed: () async {
                    await controlApiCreateMemberDialog(context, call);
                  }),
              SizedBox(height: 10),
              TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: Text('Endpoint'),
                  onPressed: () async {
                    await controlApiCreateEndpointDialog(context, call);
                  }),
            ]);
          },
        ),
      );
    },
  );
}

Future controlApiCreateRoomDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      var roomId = '';
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(mainAxisSize: MainAxisSize.min, children: [
              Flexible(
                  child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Flexible(
                    child: Text('local://'),
                  ),
                  SizedBox(width: 10),
                  Expanded(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newRoomId) async {
                        roomId = newRoomId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Room ID',
                      ),
                    ),
                  ),
                ],
              )),
              SizedBox(height: 10),
              TextButton(
                  onPressed: () async {
                    await call.controlApi.createRoom(roomId);
                    Navigator.pop(context);
                  },
                  child: Text('Create'))
            ]);
          },
        ),
      );
    },
  );
}

Future controlApiCreateMemberDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      var roomId = '';
      var memberId = '';
      var credentials = '';
      var idle = '10s';
      var reconnectTimeout = '10s';
      var ping = '3s';

      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(mainAxisSize: MainAxisSize.min, children: [
              Flexible(
                  child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Flexible(
                    child: Text('local://'),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newRoomId) async {
                        roomId = newRoomId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Room ID',
                      ),
                    ),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newMemberId) async {
                        memberId = newMemberId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Member ID',
                      ),
                    ),
                  ),
                ],
              )),
              SizedBox(height: 10),
              Flexible(
                child: Text('Credentials'),
              ),
              Flexible(
                child: TextFormField(
                  initialValue: null,
                  onChanged: (newCredentials) async {
                    credentials = newCredentials;
                  },
                  decoration: InputDecoration(
                    hintText: 'Credentials',
                  ),
                ),
              ),
              SizedBox(height: 10),
              Flexible(
                child: Text('Timeout'),
              ),
              Flexible(
                  child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  SizedBox(width: 10),
                  Flexible(
                    child: Text('IDLE'),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newIdle) async {
                        idle = newIdle;
                      },
                      decoration: InputDecoration(
                        hintText: '10s',
                      ),
                    ),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: Text('Reconnect'),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newReconnect) async {
                        reconnectTimeout = newReconnect;
                      },
                      decoration: InputDecoration(
                        hintText: '10s',
                      ),
                    ),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: Text('Ping'),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newPing) async {
                        ping = newPing;
                      },
                      decoration: InputDecoration(
                        hintText: '3s',
                      ),
                    ),
                  ),
                ],
              )),
              SizedBox(height: 10),
              TextButton(
                  onPressed: () async {
                    var member = Member(memberId, {}, Plain(credentials),
                        'grpc://127.0.0.1:9099', 'grpc://127.0.0.1:9099');

                    member.idle_timeout = idle;
                    member.reconnect_timeout = reconnectTimeout;
                    member.ping_interval = ping;

                    await call.controlApi.createMember(roomId, member);
                    Navigator.pop(context);
                  },
                  child: Text('Create'))
            ]);
          },
        ),
      );
    },
  );
}

Future controlApiCreateEndpointDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      var roomId = '';
      var memberId = '';
      var endpointId = '';
      var URL = '';
      var forceRelay = false;
      var endpointType = 'PlayEndpoint';

      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(mainAxisSize: MainAxisSize.min, children: [
              SizedBox(height: 10),
              Text('Endpoint URI'),
              Flexible(
                  child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Flexible(
                    child: Text('local://'),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newRoomId) async {
                        roomId = newRoomId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Room ID',
                      ),
                    ),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newMemberId) async {
                        memberId = newMemberId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Member ID',
                      ),
                    ),
                  ),
                  SizedBox(width: 10),
                  Flexible(
                    child: TextFormField(
                      initialValue: null,
                      onChanged: (newEndpointId) async {
                        endpointId = newEndpointId;
                      },
                      decoration: InputDecoration(
                        hintText: 'Endpoint ID',
                      ),
                    ),
                  ),
                ],
              )),
              SizedBox(height: 10),
              Flexible(
                child: Text('Endpoint type'),
              ),
              Flexible(
                child: DropdownButton<String>(
                  value: endpointType,
                  icon: const Icon(Icons.arrow_downward),
                  elevation: 16,
                  style: const TextStyle(color: Colors.deepPurple),
                  underline: Container(
                    height: 2,
                    color: Colors.deepPurpleAccent,
                  ),
                  onChanged: (String? value) {
                    endpointType = value!;
                  },
                  items: ['PlayEndpoint', 'PublishEndpoint']
                      .map<DropdownMenuItem<String>>((value) {
                    return DropdownMenuItem<String>(
                      value: value,
                      child: Text(value),
                    );
                  }).toList(),
                ),
              ),
              SizedBox(height: 10),
              Flexible(
                child: Text('Source URI'),
              ),
              Flexible(
                  child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                    SizedBox(width: 10),
                    Flexible(
                      child: Text('local://'),
                    ),
                    SizedBox(width: 10),
                    Flexible(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newUrl) async {
                          URL = newUrl;
                        },
                        decoration: InputDecoration(
                          hintText: 'roomId/memberId/sourceId',
                        ),
                      ),
                    ),
                  ])),
              SizedBox(height: 10),
              Flexible(
                child: SwitchListTile(
                    title: Text('Force relay'),
                    value: forceRelay,
                    onChanged: (v) => setStateSb(() {
                          forceRelay = v;
                        })),
              ),
              SizedBox(height: 10),
              TextButton(
                  onPressed: () async {
                    if (endpointType == 'PlayEndpoint') {
                      var endpoint = WebRtcPlayEndpoint(endpointId, URL);
                      await call.controlApi
                          .createPlayEndpoint(roomId, memberId, endpoint);
                    } else {
                      var endpoint =
                          WebRtcPublishEndpoint(endpointId, P2pMode.Always);
                      await call.controlApi
                          .createPublishEndpoint(roomId, memberId, endpoint);
                    }
                    Navigator.pop(context);
                  },
                  child: Text('Create'))
            ]);
          },
        ),
      );
    },
  );
}

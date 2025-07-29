// ignore_for_file: use_build_context_synchronously

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
  const CallRoute(
    this.roomId,
    this.memberId,
    this.isSFUMode,
    this.isPublish,
    this.publishVideo,
    this.publishAudio,
    this.fakeMedia, {
    super.key,
  });

  final String roomId;
  final String memberId;
  final bool isSFUMode;
  final bool isPublish;
  final bool publishAudio;
  final bool publishVideo;
  final bool fakeMedia;

  @override
  State<CallRoute> createState() => _CallState();
}

class ConnectWidgets {
  Text name = const Text('');
  Map<String, VideoView> videoTracks = {};
  List<TextButton> toggleButtons = List.empty(growable: true);
  bool recvVideoDevice = true;
  bool recvVideoDisplay = true;
  bool recvAudio = true;

  List<Widget> all() {
    List<Widget> res = List.empty(growable: true);

    res.add(name);

    List<Widget> buttons = toggleButtons;
    res.addAll(buttons);

    List<Widget> videos = videoTracks.values
        .map((video) => Expanded(child: video))
        .toList();
    res.addAll(videos);

    return res;
  }
}

class _CallState extends State<CallRoute> {
  late bool _isSFUMode;
  late bool _isPublish;
  late bool _publishAudio;
  late bool _publishVideo;
  late bool _fakeMedia;

  bool _videoEnabled = true;
  bool _audioEnabled = true;

  VideoView? localScreenVideo;
  VideoView? localCameraVideo;

  double currentAudioLevel = 0.0;

  final Map<String, ConnectWidgets> _widgets = {};

  Call? _call;
  late String _roomId;
  late String _memberId;

  @override
  void initState() {
    super.initState();

    _roomId = widget.roomId;
    _memberId = widget.memberId;
    _isSFUMode = widget.isSFUMode;
    _isPublish = widget.isPublish;
    _publishVideo = widget.publishVideo;
    _publishAudio = widget.publishAudio;
    _fakeMedia = widget.fakeMedia;

    () async {
      var call = await Call.create();
      call.onNewRemoteStream((track, remoteId, conn) async {
        final trackId = track.getTrack().id();
        if (track.mediaDirection() == jason.TrackMediaDirection.sendRecv) {
          var renderer = createVideoRenderer();
          await renderer.initialize();
          await renderer.setSrcObject(track.getTrack());

          var connectionWidgets = _widgets[remoteId];

          if (connectionWidgets == null) {
            connectionWidgets = ConnectWidgets();
            connectionWidgets.videoTracks = Map.from({
              trackId: VideoView(renderer),
            });
            connectionWidgets.name = Text(remoteId);
            connectionWidgets.toggleButtons = [
              TextButton(
                onPressed: () {
                  if (!connectionWidgets!.recvVideoDevice) {
                    conn.enableRemoteVideo(jason.MediaSourceKind.device);
                  } else {
                    conn.disableRemoteVideo(jason.MediaSourceKind.device);
                  }
                  connectionWidgets.recvVideoDevice =
                      !connectionWidgets.recvVideoDevice;
                },
                child: const Text('Toggle device video recv'),
              ),
              TextButton(
                onPressed: () {
                  if (!connectionWidgets!.recvVideoDisplay) {
                    conn.enableRemoteVideo(jason.MediaSourceKind.display);
                  } else {
                    conn.disableRemoteVideo(jason.MediaSourceKind.display);
                  }
                  connectionWidgets.recvVideoDisplay =
                      !connectionWidgets.recvVideoDisplay;
                },
                child: const Text('Toggle display video recv'),
              ),
              TextButton(
                onPressed: () {
                  if (!connectionWidgets!.recvAudio) {
                    conn.enableRemoteAudio();
                  } else {
                    conn.disableRemoteAudio();
                  }
                  connectionWidgets.recvAudio = !connectionWidgets.recvAudio;
                },
                child: const Text('Toggle audio recv'),
              ),
            ];
          } else {
            connectionWidgets.videoTracks[trackId] = VideoView(renderer);
          }

          setState(() {
            _widgets[remoteId] = connectionWidgets!;
          });
        }
        track.onMediaDirectionChanged((newDir) async {
          var remoteTracks = _widgets[remoteId];

          if (newDir == jason.TrackMediaDirection.sendRecv) {
            var renderer = createVideoRenderer();
            await renderer.initialize();
            await renderer.setSrcObject(track.getTrack());

            if (remoteTracks == null) {
              remoteTracks = ConnectWidgets();
              remoteTracks.videoTracks = Map.from({
                trackId: VideoView(renderer),
              });
            } else {
              remoteTracks.videoTracks[trackId] = VideoView(renderer);
            }
          } else {
            if (remoteTracks != null) {
              remoteTracks.videoTracks.remove(trackId);
            }
          }

          setState(() {
            _widgets[remoteId] = remoteTracks!;
          });
        });
      });

      call.onLocalAudioTrack((track) {
        if (track.isOnAudioLevelAvailable()) {
          track.onAudioLevelChanged((volume) {
            setState(() {
              currentAudioLevel = volume / 100;
            });
          });
        }
      });
      call.onLocalDeviceStream((track) async {
        if (localCameraVideo == null) {
          var renderer = createVideoRenderer();
          await renderer.initialize();
          await renderer.setSrcObject(track);
          localCameraVideo = VideoView(renderer, mirror: true);

          var localTracks = _widgets['I'];
          if (localTracks == null) {
            localTracks = ConnectWidgets();
            localTracks.videoTracks = Map.from({'I': localCameraVideo!});
          } else {
            localTracks.videoTracks['I'] = localCameraVideo!;
          }
          setState(() {
            _widgets['I'] = localTracks!;
          });
        } else {
          await localCameraVideo!.videoRenderer.setSrcObject(track);
        }
      });

      call.onLocalDisplayStream((track) async {
        if (localScreenVideo == null) {
          var renderer = createVideoRenderer();
          await renderer.initialize();
          await renderer.setSrcObject(track);
          localScreenVideo = VideoView(renderer, mirror: true);

          var localTracks = _widgets['I'];
          if (localTracks == null) {
            localTracks = ConnectWidgets();
            localTracks.videoTracks = Map.from({'I': localScreenVideo!});
          } else {
            localTracks.videoTracks['I'] = localScreenVideo!;
          }
          setState(() {
            _widgets['I'] = localTracks!;
          });
        } else {
          await localScreenVideo!.videoRenderer.setSrcObject(track);
        }
      });
      call.onDeviceChange(() {
        var snackBar = const SnackBar(content: Text('On device change'));
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      });

      call.onError((err) {
        var snackBar = SnackBar(content: Text(err));
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      });

      call.start(
        _roomId,
        _memberId,
        _isPublish,
        _publishVideo,
        _publishAudio,
        _fakeMedia,
        _isSFUMode,
      );

      setState(() {
        _call = call;
      });
    }();
  }

  @override
  Widget build(BuildContext context) {
    if (_call == null) {
      return const SizedBox.shrink();
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Medea call demo'),
        actions: <Widget>[
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.white,
              backgroundColor: Colors.blue,
            ),
            child: const Text('MediaSetting'),
            onPressed: () async {
              await mediaSettingDialog(context, _call!);
            },
          ),
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.white,
              backgroundColor: Colors.blue,
            ),
            child: const Text('Create'),
            onPressed: () async {
              await controlApiCreateDialog(context, _call!);
            },
          ),
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.white,
              backgroundColor: Colors.blue,
            ),
            child: const Text('Get'),
            onPressed: () async {
              await controlApiGetDialog(context, _call!);
            },
          ),
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.white,
              backgroundColor: Colors.blue,
            ),
            child: const Text('Delete'),
            onPressed: () async {
              await controlApiDeleteDialog(context, _call!);
            },
          ),
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.white,
              backgroundColor: Colors.blue,
            ),
            child: const Text('Callbacks'),
            onPressed: () async {
              await showCallbacks(context, _call!);
            },
          ),
        ],
      ),
      body: Center(
        child: SizedBox(
          width: MediaQuery.of(context).size.width,
          height: MediaQuery.of(context).size.height,
          child: Column(
            children: [
              LinearProgressIndicator(
                value: currentAudioLevel,
                minHeight: 10.0,
              ),
              Expanded(
                child: Row(
                  children: _widgets.values
                      .map(
                        (videoMap) => Expanded(
                          child: Column(children: videoMap.all().toList()),
                        ),
                      )
                      .toList(),
                ),
              ),
            ],
          ),
        ),
      ),
      floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
      floatingActionButton: Padding(
        padding: const EdgeInsets.only(bottom: 50.0),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Padding(
              padding: const EdgeInsets.only(right: 30.0),
              child: FloatingActionButton(
                onPressed: () async {
                  setState(() {
                    _audioEnabled = !_audioEnabled;
                  });

                  await _call!.toggleAudio(_audioEnabled);
                },
                heroTag: null,
                child: Icon(_audioEnabled ? Icons.mic_off : Icons.mic),
              ),
            ),
            Padding(
              padding: const EdgeInsets.only(right: 30.0),
              child: FloatingActionButton(
                onPressed: () async {
                  setState(() {
                    _videoEnabled = !_videoEnabled;
                  });

                  await _call!.toggleVideo(_videoEnabled);
                },
                heroTag: null,
                child: Icon(
                  _videoEnabled ? Icons.videocam_off : Icons.videocam,
                ),
              ),
            ),
            FloatingActionButton(
              onPressed: () async {
                await _call!.dispose();
                Navigator.pop(context);
              },
              heroTag: null,
              backgroundColor: Colors.red,
              child: const Icon(Icons.call_end),
            ),
          ],
        ),
      ),
    );
  }
}

Future<void> showCallbacks(BuildContext context, Call call) async {
  final cbs = await call.controlApi.getCallbacks();

  await showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: SizedBox(
          width: double.maxFinite,
          child: ListView(
            shrinkWrap: true,
            children: cbs
                .map(
                  (cb) => Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Expanded(child: Text(cb.event.toJson().toString())),
                      Expanded(child: Text(cb.at)),
                      Expanded(child: Text(cb.fid)),
                    ],
                  ),
                )
                .toList(),
          ),
        ),
      );
    },
  );
}

Future<void> controlApiGetDialog(BuildContext context, Call call) async {
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
                    const Flexible(child: Text('local://')),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newRoomId) async {
                          roomId = newRoomId;
                        },
                        decoration: const InputDecoration(hintText: 'Room ID'),
                      ),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newMemberId) async {
                          memberId = newMemberId;
                        },
                        decoration: const InputDecoration(
                          hintText: 'Member ID',
                        ),
                      ),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newEndpointId) async {
                          endpointId = newEndpointId;
                        },
                        decoration: const InputDecoration(
                          hintText: 'Endpoint ID',
                        ),
                      ),
                    ),
                    const SizedBox(width: 10),
                  ],
                ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () async {
                    await call.controlApi.get(roomId, memberId, endpointId);

                    Navigator.pop(context);
                  },
                  child: const Text('Get'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> controlApiDeleteDialog(BuildContext context, Call call) async {
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
                    const Flexible(child: Text('local://')),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newRoomId) async {
                          roomId = newRoomId;
                        },
                        decoration: const InputDecoration(hintText: 'Room ID'),
                      ),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newMemberId) async {
                          memberId = newMemberId;
                        },
                        decoration: const InputDecoration(
                          hintText: 'Member ID',
                        ),
                      ),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextFormField(
                        initialValue: null,
                        onChanged: (newEndpointId) async {
                          endpointId = newEndpointId;
                        },
                        decoration: const InputDecoration(
                          hintText: 'Endpoint ID',
                        ),
                      ),
                    ),
                    const SizedBox(width: 10),
                  ],
                ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () async {
                    await call.controlApi.delete(roomId, memberId, endpointId);
                    Navigator.pop(context);
                  },
                  child: const Text('Get'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> mediaSettingDialog(BuildContext context, Call call) async {
  final displayList = await call.enumerateDisplay();

  final deviceList = await call.enumerateDevice();
  final videoDevices = deviceList
      .where((element) => element.kind() == jason.MediaDeviceKind.videoInput)
      .toList();
  final audioDevices = deviceList
      .where((element) => element.kind() == jason.MediaDeviceKind.audioInput)
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
                  title: const Text('Screen share'),
                  value: call.screenShare,
                  onChanged: (v) => setStateSb(() {
                    call.screenShare = v;
                  }),
                ),
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
                      child: Text(
                        value.title() == null
                            ? value.deviceId()
                            : value.title()!,
                      ),
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
                    } catch (e) {
                      // No-op.
                    }
                  },
                  decoration: const InputDecoration(labelText: 'Display FPS'),
                ),
                TextFormField(
                  initialValue: call.selectedDisplayWidth == null
                      ? '640'
                      : call.selectedDisplayWidth.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDisplayWidth = int.parse(text);
                    } catch (e) {
                      // No-op.
                    }
                  },
                  decoration: const InputDecoration(labelText: 'Display width'),
                ),
                TextFormField(
                  initialValue: call.selectedDisplayHeight == null
                      ? '480'
                      : call.selectedDisplayHeight.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDisplayHeight = int.parse(text);
                    } catch (e) {
                      // No-op.
                    }
                  },
                  decoration: const InputDecoration(
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
                    } catch (e) {
                      // No-op.
                    }
                  },
                  decoration: const InputDecoration(labelText: 'Device width'),
                ),
                TextFormField(
                  initialValue: call.selectedDeviceHeight == null
                      ? '480'
                      : call.selectedDeviceHeight.toString(),
                  keyboardType: TextInputType.number,
                  onChanged: (text) {
                    try {
                      call.selectedDeviceHeight = int.parse(text);
                    } catch (e) {
                      // No-op.
                    }
                  },
                  decoration: const InputDecoration(labelText: 'Device height'),
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
                      child: Text(
                        '${_audioSend ? 'Disable' : 'Enable'} Audio Send',
                      ),
                    ),
                    TextButton(
                      onPressed: () async {
                        await call.setSendVideo(!_videoSend);
                        setStateSb(() {
                          _videoSend = !_videoSend;
                        });
                      },
                      child: Text(
                        '${_videoSend ? 'Disable' : 'Enable'} Video Send',
                      ),
                    ),
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
                      child: Text(
                        '${_audioRecv ? 'Disable' : 'Enable'} Audio Recv',
                      ),
                    ),
                    TextButton(
                      onPressed: () async {
                        await call.setRecvVideo(!_videoRecv);
                        setStateSb(() {
                          _videoRecv = !_videoRecv;
                        });
                      },
                      child: Text(
                        '${_videoRecv ? 'Disable' : 'Enable'} Video Recv',
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 10),
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
                      displayTrack.exactFrameRate(
                        call.selectedDisplayFrameRate!,
                      );
                    }

                    var audioTrack = jason.AudioTrackConstraints();
                    audioTrack.deviceId(call.audioDeviceId!);

                    var displayAudioTrack = jason.AudioTrackConstraints();

                    await call.setMedia(
                      videoTrack,
                      audioTrack,
                      displayTrack,
                      displayAudioTrack,
                    );
                  },
                  child: const Text('Set media setting'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> controlApiCreateDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: const Text('Room'),
                  onPressed: () async {
                    await controlApiCreateRoomDialog(context, call);
                  },
                ),
                const SizedBox(height: 10),
                TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: const Text('Member'),
                  onPressed: () async {
                    await controlApiCreateMemberDialog(context, call);
                  },
                ),
                const SizedBox(height: 10),
                TextButton(
                  style: TextButton.styleFrom(
                    foregroundColor: Colors.white,
                    backgroundColor: Colors.blue,
                  ),
                  child: const Text('Endpoint'),
                  onPressed: () async {
                    await controlApiCreateEndpointDialog(context, call);
                  },
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> controlApiCreateRoomDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      var roomId = '';
      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Flexible(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Flexible(child: Text('local://')),
                      const SizedBox(width: 10),
                      Expanded(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newRoomId) => roomId = newRoomId,
                          decoration: const InputDecoration(
                            hintText: 'Room ID',
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () async {
                    await call.controlApi.createRoom(roomId);
                    Navigator.pop(context);
                  },
                  child: const Text('Create'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> controlApiCreateMemberDialog(BuildContext context, Call call) {
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
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Flexible(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Flexible(child: Text('local://')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newRoomId) {
                            roomId = newRoomId;
                          },
                          decoration: const InputDecoration(
                            hintText: 'Room ID',
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newMemberId) {
                            memberId = newMemberId;
                          },
                          decoration: const InputDecoration(
                            hintText: 'Member ID',
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 10),
                const Flexible(child: Text('Credentials')),
                Flexible(
                  child: TextFormField(
                    initialValue: null,
                    onChanged: (newCredentials) async {
                      credentials = newCredentials;
                    },
                    decoration: const InputDecoration(hintText: 'Credentials'),
                  ),
                ),
                const SizedBox(height: 10),
                const Flexible(child: Text('Timeout')),
                Flexible(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const SizedBox(width: 10),
                      const Flexible(child: Text('IDLE')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newIdle) async {
                            idle = newIdle;
                          },
                          decoration: const InputDecoration(hintText: '10s'),
                        ),
                      ),
                      const SizedBox(width: 10),
                      const Flexible(child: Text('Reconnect')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newReconnect) async {
                            reconnectTimeout = newReconnect;
                          },
                          decoration: const InputDecoration(hintText: '10s'),
                        ),
                      ),
                      const SizedBox(width: 10),
                      const Flexible(child: Text('Ping')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newPing) async {
                            ping = newPing;
                          },
                          decoration: const InputDecoration(hintText: '3s'),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () async {
                    final member = Member(
                      memberId,
                      {},
                      Plain(credentials),
                      'grpc://127.0.0.1:9099',
                      'grpc://127.0.0.1:9099',
                    );

                    member.idle_timeout = idle;
                    member.reconnect_timeout = reconnectTimeout;
                    member.ping_interval = ping;

                    await call.controlApi.createMember(roomId, member);
                    Navigator.pop(context);
                  },
                  child: const Text('Create'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

Future<void> controlApiCreateEndpointDialog(BuildContext context, Call call) {
  return showDialog<void>(
    context: context,
    builder: (BuildContext context) {
      var roomId = '';
      var memberId = '';
      var endpointId = '';
      var url = '';
      var forceRelay = false;
      var isSFUMode = true;
      var endpointType = 'PlayEndpoint';

      return AlertDialog(
        content: StatefulBuilder(
          builder: (BuildContext context, StateSetter setStateSb) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const SizedBox(height: 10),
                const Text('Endpoint URI'),
                Flexible(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Flexible(child: Text('local://')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newRoomId) async {
                            roomId = newRoomId;
                          },
                          decoration: const InputDecoration(
                            hintText: 'Room ID',
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newMemberId) async {
                            memberId = newMemberId;
                          },
                          decoration: const InputDecoration(
                            hintText: 'Member ID',
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newEndpointId) async {
                            endpointId = newEndpointId;
                          },
                          decoration: const InputDecoration(
                            hintText: 'Endpoint ID',
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 10),
                const Flexible(child: Text('Endpoint type')),
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
                        })
                        .toList(),
                  ),
                ),
                const SizedBox(height: 10),
                const Flexible(child: Text('Source URI')),
                Flexible(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const SizedBox(width: 10),
                      const Flexible(child: Text('local://')),
                      const SizedBox(width: 10),
                      Flexible(
                        child: TextFormField(
                          initialValue: null,
                          onChanged: (newUrl) async {
                            url = newUrl;
                          },
                          decoration: const InputDecoration(
                            hintText: 'roomId/memberId/sourceId',
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 10),
                Flexible(
                  child: SwitchListTile(
                    title: const Text('SFU mode'),
                    value: isSFUMode,
                    onChanged: (v) => setStateSb(() => isSFUMode = v),
                  ),
                ),
                if (!isSFUMode)
                  Flexible(
                    child: SwitchListTile(
                      title: const Text('Force relay'),
                      value: forceRelay,
                      onChanged: (v) => setStateSb(() => forceRelay = v),
                    ),
                  ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () async {
                    if (endpointType == 'PlayEndpoint') {
                      final endpoint = WebRtcPlayEndpoint(endpointId, url);
                      await call.controlApi.createPlayEndpoint(
                        roomId,
                        memberId,
                        endpoint,
                      );
                    } else {
                      final endpoint = WebRtcPublishEndpoint(
                        endpointId,
                        isSFUMode ? P2pMode.Never : P2pMode.Always,
                      );
                      await call.controlApi.createPublishEndpoint(
                        roomId,
                        memberId,
                        endpoint,
                      );
                    }
                    Navigator.pop(context);
                  },
                  child: const Text('Create'),
                ),
              ],
            );
          },
        ),
      );
    },
  );
}

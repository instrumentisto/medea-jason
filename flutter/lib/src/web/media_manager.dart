import '../interface/local_media_track.dart';
import '../interface/input_device_info.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';
import '../web/input_device_info.dart';
import '../web/local_media_track.dart';
import '../web/media_stream_settings.dart';

class WebMediaManagerHandle extends MediaManagerHandle {
  late wasm.MediaManagerHandle obj;

  WebMediaManagerHandle(this.obj);

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      IMediaStreamSettings caps) async {
    var tracks = await obj.init_local_tracks((caps as MediaStreamSettings).obj);
    return tracks.map((t) => WebLocalMediaTrack(t)).toList();
  }

  @override
  Future<List<InputDeviceInfo>> enumerateDevices() async {
    var tracks = await obj.enumerate_devices();
    return tracks.map((t) => WebInputDeviceInfo(t)).toList();
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}

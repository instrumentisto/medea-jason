
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0;
function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0;
function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0;
function getFloat64Memory0() {
    if (cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0df6dc30138d5bb1(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_43(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb60990723a42bfe3(arg0, arg1, addHeapObject(arg2));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
function __wbg_adapter_348(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h17f6ca5a16113e2b(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
* Possible error kinds of a [`LocalMediaInitException`].
*/
export const LocalMediaInitExceptionKind = Object.freeze({
/**
* Occurs if the [getUserMedia()][1] request failed.
*
* [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
*/
GetUserMediaFailed:0,"0":"GetUserMediaFailed",
/**
* Occurs if the [getUserMedia()][1] request failed on getting audio
* track.
*
* [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
*/
GetUserMediaAudioFailed:1,"1":"GetUserMediaAudioFailed",
/**
* Occurs if the [getUserMedia()][1] request failed on getting video
* track.
*
* [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
*/
GetUserMediaVideoFailed:2,"2":"GetUserMediaVideoFailed",
/**
* Occurs if the [getDisplayMedia()][1] request failed.
*
* [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
*/
GetDisplayMediaFailed:3,"3":"GetDisplayMediaFailed",
/**
* Occurs when local track is [`ended`][1] right after [getUserMedia()][2]
* or [getDisplayMedia()][3] request.
*
* [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
* [2]: https://tinyurl.com/rnxcavf
* [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
*/
LocalTrackIsEnded:4,"4":"LocalTrackIsEnded", });
/**
* Possible error kinds of a [`RpcClientException`].
*/
export const RpcClientExceptionKind = Object.freeze({
/**
* Connection with a server was lost.
*
* This usually means that some transport error occurred, so a client can
* continue performing reconnecting attempts.
*/
ConnectionLost:0,"0":"ConnectionLost",
/**
* Could not authorize an RPC session.
*
* This usually means that authentication data a client provides is
* obsolete.
*/
AuthorizationFailed:1,"1":"AuthorizationFailed",
/**
* RPC session has been finished. This is a terminal state.
*/
SessionFinished:2,"2":"SessionFinished", });
/**
* Kind of a [`MediaStateTransitionException`].
*/
export const MediaStateTransitionExceptionKind = Object.freeze({
/**
* Media state of a [`Sender`] transits to an opposite of the requested
* one.
*
* [`Sender`]: crate::peer::media::Sender
*/
OppositeState:0,"0":"OppositeState",
/**
* Requested state transition is not allowed by [`Sender`]'s settings.
*
* [`Sender`]: crate::peer::media::Sender
*/
ProhibitedState:1,"1":"ProhibitedState", });
/**
* Media exchange direction of a `Track`.
*/
export const MediaDirection = Object.freeze({
/**
* `Track` is enabled on recv and send sides.
*/
SendRecv:0,"0":"SendRecv",
/**
* `Track` is enabled on send side.
*/
SendOnly:1,"1":"SendOnly",
/**
* `Track` is enabled on recv side.
*/
RecvOnly:2,"2":"RecvOnly",
/**
* `Track` is disabled on both sides.
*/
Inactive:3,"3":"Inactive", });
/**
* Describes directions that a camera can face, as seen from a user's
* perspective. Representation of a [VideoFacingModeEnum][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
*/
export const FacingMode = Object.freeze({
/**
* Facing towards a user (a self-view camera).
*/
User:0,"0":"User",
/**
* Facing away from a user (viewing the environment).
*/
Environment:1,"1":"Environment",
/**
* Facing to the left of a user.
*/
Left:2,"2":"Left",
/**
* Facing to the right of a user.
*/
Right:3,"3":"Right", });
/**
* Media source type.
*/
export const MediaSourceKind = Object.freeze({
/**
* Media is sourced from some media device (webcam or microphone).
*/
Device:0,"0":"Device",
/**
* Media is obtained via screen capturing.
*/
Display:1,"1":"Display", });
/**
* [MediaDeviceInfo.kind][1] representation.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
*/
export const MediaDeviceKind = Object.freeze({
/**
* Audio input device (for example, a microphone).
*/
AudioInput:0,"0":"AudioInput",
/**
* Video input device (for example, a webcam).
*/
VideoInput:1,"1":"VideoInput",
/**
* Audio output device (for example, a pair of headphones).
*/
AudioOutput:2,"2":"AudioOutput", });
/**
* [MediaStreamTrack.kind][1] representation.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
*/
export const MediaKind = Object.freeze({
/**
* Audio track.
*/
Audio:0,"0":"Audio",
/**
* Video track.
*/
Video:1,"1":"Video", });
/**
* Constraints applicable to audio tracks.
*/
export class AudioTrackConstraints {

    static __wrap(ptr) {
        const obj = Object.create(AudioTrackConstraints.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_audiotrackconstraints_free(ptr);
    }
    /**
    * Creates new [`AudioTrackConstraints`] with none constraints configured.
    */
    constructor() {
        const ret = wasm.audiotrackconstraints_new();
        return AudioTrackConstraints.__wrap(ret);
    }
    /**
    * Sets an exact [deviceId][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    * @param {string} device_id
    */
    device_id(device_id) {
        const ptr0 = passStringToWasm0(device_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.audiotrackconstraints_device_id(this.ptr, ptr0, len0);
    }
}
/**
* Connection with a specific remote `Member`, that is used on JS side.
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*/
export class ConnectionHandle {

    static __wrap(ptr) {
        const obj = Object.create(ConnectionHandle.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_connectionhandle_free(ptr);
    }
    /**
    * Sets callback, invoked when this [`Connection`] is closed.
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    * a [`RoomHandle`] that implicitly owns native object behind this
    * [`ConnectionHandle`].
    *
    * [`Connection`]: connection::Connection
    * [`Jason`]: api::Jason
    * [`RoomHandle`]: api::RoomHandle
    * [`StateError`]: api::err::StateError
    * @param {Function} cb
    */
    on_close(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.connectionhandle_on_close(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns ID of the remote `Member`.
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    * a [`RoomHandle`] that implicitly owns native object behind this
    * [`ConnectionHandle`].
    *
    * [`Jason`]: api::Jason
    * [`RoomHandle`]: api::RoomHandle
    * [`StateError`]: crate::api::err::StateError
    * @returns {string}
    */
    get_remote_member_id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.connectionhandle_get_remote_member_id(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr0 = r0;
            var len0 = r1;
            if (r3) {
                ptr0 = 0; len0 = 0;
                throw takeObject(r2);
            }
            return getStringFromWasm0(ptr0, len0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(ptr0, len0);
        }
    }
    /**
    * Sets callback, invoked when a new [`RemoteMediaTrack`] is added to this
    * [`Connection`].
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    * a [`RoomHandle`] that implicitly owns native object behind this
    * [`ConnectionHandle`].
    *
    * [`Connection`]: connection::Connection
    * [`Jason`]: api::Jason
    * [`RemoteMediaTrack`]: crate::api::RemoteMediaTrack
    * [`RoomHandle`]: api::RoomHandle
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_remote_track_added(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.connectionhandle_on_remote_track_added(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets callback, invoked when connection quality score is updated by a
    * server.
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
    * a [`RoomHandle`] that implicitly owns native object behind this
    * [`ConnectionHandle`].
    *
    * [`Jason`]: api::Jason
    * [`RoomHandle`]: api::RoomHandle
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_quality_score_update(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.connectionhandle_on_quality_score_update(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Enables inbound audio in this [`ConnectionHandle`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`ConnectionHandle::disable_remote_video()`] was called while enabling
    * or a media server didn't approve this state transition.
    *
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    enable_remote_audio() {
        const ret = wasm.connectionhandle_enable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Disables inbound audio in this [`ConnectionHandle`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`ConnectionHandle::enable_remote_video()`] was called while disabling
    * or a media server didn't approve this state transition.
    *
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    disable_remote_audio() {
        const ret = wasm.connectionhandle_disable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Enables inbound video in this [`ConnectionHandle`].
    *
    * Affects only video with the specific [`MediaSourceKind`], if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`ConnectionHandle::disable_remote_audio()`] was called while enabling
    * or a media server didn't approve this state transition.
    *
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    enable_remote_video(source_kind) {
        const ret = wasm.connectionhandle_enable_remote_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Disables inbound video in this [`ConnectionHandle`].
    *
    * Affects only video with the specific [`MediaSourceKind`], if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`ConnectionHandle::enable_remote_audio()`] was called while disabling
    * or a media server didn't approve this state transition.
    *
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    disable_remote_video(source_kind) {
        const ret = wasm.connectionhandle_disable_remote_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
}
/**
* Constraints applicable to video tracks that are sourced from some media
* device.
*/
export class DeviceVideoTrackConstraints {

    static __wrap(ptr) {
        const obj = Object.create(DeviceVideoTrackConstraints.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_devicevideotrackconstraints_free(ptr);
    }
    /**
    * Creates new [`DeviceVideoTrackConstraints`] with none constraints
    * configured.
    */
    constructor() {
        const ret = wasm.devicevideotrackconstraints_new();
        return DeviceVideoTrackConstraints.__wrap(ret);
    }
    /**
    * Sets an exact [deviceId][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    * @param {string} device_id
    */
    device_id(device_id) {
        const ptr0 = passStringToWasm0(device_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.audiotrackconstraints_device_id(this.ptr, ptr0, len0);
    }
    /**
    * Sets an exact [facingMode][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
    * @param {number} facing_mode
    */
    exact_facing_mode(facing_mode) {
        wasm.devicevideotrackconstraints_exact_facing_mode(this.ptr, facing_mode);
    }
    /**
    * Sets an ideal [facingMode][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
    * @param {number} facing_mode
    */
    ideal_facing_mode(facing_mode) {
        wasm.devicevideotrackconstraints_ideal_facing_mode(this.ptr, facing_mode);
    }
    /**
    * Sets an exact [`height`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-height
    * @param {number} height
    */
    exact_height(height) {
        wasm.devicevideotrackconstraints_exact_height(this.ptr, height);
    }
    /**
    * Sets an ideal [`height`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-height
    * @param {number} height
    */
    ideal_height(height) {
        wasm.devicevideotrackconstraints_ideal_height(this.ptr, height);
    }
    /**
    * Sets a range of a [`height`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-height
    * @param {number} min
    * @param {number} max
    */
    height_in_range(min, max) {
        wasm.devicevideotrackconstraints_height_in_range(this.ptr, min, max);
    }
    /**
    * Sets an exact [`width`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-width
    * @param {number} width
    */
    exact_width(width) {
        wasm.devicevideotrackconstraints_exact_width(this.ptr, width);
    }
    /**
    * Sets an ideal [`width`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-width
    * @param {number} width
    */
    ideal_width(width) {
        wasm.devicevideotrackconstraints_ideal_width(this.ptr, width);
    }
    /**
    * Sets a range of a [`width`][1] constraint.
    *
    * [1]: https://tinyurl.com/w3-streams#def-constraint-width
    * @param {number} min
    * @param {number} max
    */
    width_in_range(min, max) {
        wasm.devicevideotrackconstraints_width_in_range(this.ptr, min, max);
    }
}
/**
* Constraints applicable to video tracks sourced from a screen capturing.
*/
export class DisplayVideoTrackConstraints {

    static __wrap(ptr) {
        const obj = Object.create(DisplayVideoTrackConstraints.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_displayvideotrackconstraints_free(ptr);
    }
    /**
    * Creates new [`DisplayVideoTrackConstraints`] with none constraints
    * configured.
    */
    constructor() {
        const ret = wasm.displayvideotrackconstraints_new();
        return DisplayVideoTrackConstraints.__wrap(ret);
    }
}
/**
* Exception thrown when cannot get info of available media devices.
*/
export class EnumerateDevicesException {

    static __wrap(ptr) {
        const obj = Object.create(EnumerateDevicesException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_enumeratedevicesexception_free(ptr);
    }
    /**
    * Returns [`platform::Error`] causing this [`EnumerateDevicesException`].
    * @returns {Error}
    */
    cause() {
        const ret = wasm.enumeratedevicesexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns stacktrace of this [`EnumerateDevicesException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.enumeratedevicesexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Exception thrown when a string or some other data doesn't have an expected
* format and cannot be parsed or processed.
*/
export class FormatException {

    static __wrap(ptr) {
        const obj = Object.create(FormatException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_formatexception_free(ptr);
    }
    /**
    * Returns an error message describing of the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Jason's internal exception.
*
* This is either a programmatic error or some unexpected platform component
* failure that cannot be handled in any way.
*/
export class InternalException {

    static __wrap(ptr) {
        const obj = Object.create(InternalException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_internalexception_free(ptr);
    }
    /**
    * Returns an error message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns [`platform::Error`] causing this [`RpcClientException`].
    * @returns {Error | undefined}
    */
    cause() {
        const ret = wasm.internalexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns stacktrace of this [`InternalException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.internalexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Exception thrown when cannot change output audio device ID.
*/
export class InvalidOutputAudioDeviceIdException {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invalidoutputaudiodeviceidexception_free(ptr);
    }
    /**
    * Returns stacktrace of this [`InvalidOutputAudioDeviceIdException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.invalidoutputaudiodeviceidexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* General JS side library interface.
*
* Responsible for managing shared transports, local media and room
* initialization.
*/
export class Jason {

    static __wrap(ptr) {
        const obj = Object.create(Jason.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jason_free(ptr);
    }
    /**
    * Instantiates a new [`Jason`] interface to interact with this library.
    */
    constructor() {
        const ret = wasm.jason_new();
        return Jason.__wrap(ret);
    }
    /**
    * Creates a new `Room` and returns its [`RoomHandle`].
    * @returns {RoomHandle}
    */
    init_room() {
        const ret = wasm.jason_init_room(this.ptr);
        return RoomHandle.__wrap(ret);
    }
    /**
    * Returns a [`MediaManagerHandle`].
    * @returns {MediaManagerHandle}
    */
    media_manager() {
        const ret = wasm.jason_media_manager(this.ptr);
        return MediaManagerHandle.__wrap(ret);
    }
    /**
    * Closes the provided [`RoomHandle`].
    * @param {RoomHandle} room_to_delete
    */
    close_room(room_to_delete) {
        _assertClass(room_to_delete, RoomHandle);
        var ptr0 = room_to_delete.ptr;
        room_to_delete.ptr = 0;
        wasm.jason_close_room(this.ptr, ptr0);
    }
    /**
    * Drops [`Jason`] API object, so all the related objects (rooms,
    * connections, streams etc.) respectively. All objects related to this
    * [`Jason`] API object will be detached (you will still hold them, but
    * unable to use).
    */
    dispose() {
        const ptr = this.__destroy_into_raw();
        wasm.jason_dispose(ptr);
    }
}
/**
* Exception thrown when accessing media devices.
*/
export class LocalMediaInitException {

    static __wrap(ptr) {
        const obj = Object.create(LocalMediaInitException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_localmediainitexception_free(ptr);
    }
    /**
    * Returns concrete error kind of this [`LocalMediaInitException`].
    * @returns {number}
    */
    kind() {
        const ret = wasm.localmediainitexception_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns an error message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns [`platform::Error`] causing this [`LocalMediaInitException`].
    * @returns {Error | undefined}
    */
    cause() {
        const ret = wasm.internalexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns stacktrace of this [`LocalMediaInitException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.internalexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Wrapper around a local [MediaStreamTrack][1].
*
* Backed by a strong reference to the actual track implementing auto stop on
* dropping. Can be manually dropped with a `free()` call.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
*/
export class LocalMediaTrack {

    static __wrap(ptr) {
        const obj = Object.create(LocalMediaTrack.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_localmediatrack_free(ptr);
    }
    /**
    * Returns the underlying [MediaStreamTrack][1].
    *
    * [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
    * @returns {MediaStreamTrack}
    */
    get_track() {
        const ret = wasm.localmediatrack_get_track(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
    * audio track, or a [`MediaKind::Video`] if it represents a video track.
    * @returns {number}
    */
    kind() {
        const ret = wasm.localmediatrack_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
    * sourced from some device (webcam/microphone), or a
    * [`MediaSourceKind::Display`] if it's captured via
    * [MediaDevices.getDisplayMedia()][1].
    *
    * [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    * @returns {number}
    */
    media_source_kind() {
        const ret = wasm.localmediatrack_media_source_kind(this.ptr);
        return ret >>> 0;
    }
}
/**
* Representation of a [MediaDeviceInfo][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#device-info
*/
export class MediaDeviceInfo {

    static __wrap(ptr) {
        const obj = Object.create(MediaDeviceInfo.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediadeviceinfo_free(ptr);
    }
    /**
    * Returns a unique identifier for the represented device.
    * @returns {string}
    */
    device_id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediadeviceinfo_device_id(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns a kind of the represented device.
    *
    * This representation of [MediaDeviceInfo][1] is for input device ONLY.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#device-info
    * @returns {number}
    */
    kind() {
        const ret = wasm.mediadeviceinfo_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns label describing the represented device (for example "External
    * USB Webcam").
    *
    * If the device has no associated label, then returns an empty string.
    * @returns {string}
    */
    label() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediadeviceinfo_label(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns a group identifier of the represented device.
    *
    * Two devices have the same group identifier if they belong to the same
    * physical device. For example, the audio input and output devices
    * representing the speaker and microphone of the same headset have the
    * same [groupId][1].
    *
    * [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    * @returns {string | undefined}
    */
    group_id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediadeviceinfo_group_id(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
* [`MediaManagerHandle`] is a weak reference to a [`MediaManager`].
*
* [`MediaManager`] performs all the media acquisition requests
* ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all the received
* tracks for further re-usage.
*
* [`MediaManager`] stores weak references to [`LocalMediaTrack`]s, so if there
* are no strong references to some track, then this track is stopped and
* removed from [`MediaManager`].
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*
* [`MediaManager`]: media::MediaManager
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
* [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
*/
export class MediaManagerHandle {

    static __wrap(ptr) {
        const obj = Object.create(MediaManagerHandle.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediamanagerhandle_free(ptr);
    }
    /**
    * Returns a list of [`MediaDeviceInfo`] objects representing available
    * media input and output devices, such as microphones, cameras, and so
    * forth.
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`MediaManagerHandle`], or on a [`Jason`] that
    * implicitly owns native object behind this [`MediaManagerHandle`].
    *
    * With a [`EnumerateDevicesException`][0] if a request of platform media
    * devices access failed.
    *
    * [`Jason`]: crate::api::Jason
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::EnumerateDevicesException
    * @returns {Promise<any>}
    */
    enumerate_devices() {
        const ret = wasm.mediamanagerhandle_enumerate_devices(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns [`LocalMediaTrack`]s objects, built from the provided
    * [`MediaStreamSettings`].
    *
    * # Errors
    *
    * With a [`StateError`] if an underlying object has been disposed, e.g.
    * `free` was called on this [`MediaManagerHandle`], or on a [`Jason`] that
    * implicitly owns native object behind this [`MediaManagerHandle`].
    *
    * With a [`LocalMediaInitException`] if a request of platform media
    * devices access failed.
    *
    * [`Jason`]: crate::api::Jason
    * [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    * [`StateError`]: crate::api::err::StateError
    * @param {MediaStreamSettings} caps
    * @returns {Promise<any>}
    */
    init_local_tracks(caps) {
        _assertClass(caps, MediaStreamSettings);
        const ret = wasm.mediamanagerhandle_init_local_tracks(this.ptr, caps.ptr);
        return takeObject(ret);
    }
    /**
    * Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
    * @param {Function} cb
    */
    on_device_change(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediamanagerhandle_on_device_change(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
* Errors occurring in [`RoomHandle::set_local_media_settings()`][1] method.
*
* [1]: crate::api::RoomHandle::set_local_media_settings
*/
export class MediaSettingsUpdateException {

    static __wrap(ptr) {
        const obj = Object.create(MediaSettingsUpdateException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediasettingsupdateexception_free(ptr);
    }
    /**
    * Returns an error message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns the original [`room::ChangeMediaStateError`] that was
    * encountered while updating local media settings.
    * @returns {any}
    */
    cause() {
        const ret = wasm.mediasettingsupdateexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns whether media settings were successfully rolled back after new
    * settings application failed.
    * @returns {boolean}
    */
    rolled_back() {
        const ret = wasm.mediasettingsupdateexception_rolled_back(this.ptr);
        return ret !== 0;
    }
}
/**
* Exception thrown when the requested media state transition could not be
* performed.
*/
export class MediaStateTransitionException {

    static __wrap(ptr) {
        const obj = Object.create(MediaStateTransitionException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediastatetransitionexception_free(ptr);
    }
    /**
    * Returns an error message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns stacktrace of this [`MediaStateTransitionException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediastatetransitionexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns concrete error kind of this [`MediaStateTransitionException`].
    * @returns {number}
    */
    kind() {
        const ret = wasm.mediastatetransitionexception_kind(this.ptr);
        return ret >>> 0;
    }
}
/**
* [MediaStreamConstraints][1] wrapper.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
*/
export class MediaStreamSettings {

    static __wrap(ptr) {
        const obj = Object.create(MediaStreamSettings.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediastreamsettings_free(ptr);
    }
    /**
    * Creates new [`MediaStreamSettings`] with none constraints configured.
    */
    constructor() {
        const ret = wasm.mediastreamsettings_new();
        return MediaStreamSettings.__wrap(ret);
    }
    /**
    * Specifies the nature and settings of an audio [MediaStreamTrack][1].
    *
    * [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    * @param {AudioTrackConstraints} constraints
    */
    audio(constraints) {
        _assertClass(constraints, AudioTrackConstraints);
        var ptr0 = constraints.ptr;
        constraints.ptr = 0;
        wasm.mediastreamsettings_audio(this.ptr, ptr0);
    }
    /**
    * Set constraints that will be used to obtain a local video sourced from
    * a media device.
    * @param {DeviceVideoTrackConstraints} constraints
    */
    device_video(constraints) {
        _assertClass(constraints, DeviceVideoTrackConstraints);
        var ptr0 = constraints.ptr;
        constraints.ptr = 0;
        wasm.mediastreamsettings_device_video(this.ptr, ptr0);
    }
    /**
    * Set constraints that will be used to capture a local video from a user's
    * display.
    * @param {DisplayVideoTrackConstraints} constraints
    */
    display_video(constraints) {
        _assertClass(constraints, DisplayVideoTrackConstraints);
        var ptr0 = constraints.ptr;
        constraints.ptr = 0;
        wasm.mediastreamsettings_display_video(this.ptr, ptr0);
    }
}
/**
* Exception thrown when cannot interact with microphone volume.
*/
export class MicVolumeException {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_micvolumeexception_free(ptr);
    }
    /**
    * Returns the [`platform::Error`] causing this [`MicVolumeException`].
    * @returns {Error}
    */
    cause() {
        const ret = wasm.enumeratedevicesexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns stacktrace of this [`MicVolumeException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.enumeratedevicesexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Handle that JS side can reconnect to a media server with when a connection
* is lost.
*
* This handle is passed into a [`RoomHandle.on_connection_loss`] callback.
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*
* [`RoomHandle.on_connection_loss`]: crate::api::RoomHandle.on_connection_loss
*/
export class ReconnectHandle {

    static __wrap(ptr) {
        const obj = Object.create(ReconnectHandle.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reconnecthandle_free(ptr);
    }
    /**
    * Tries to reconnect after the provided delay in milliseconds.
    *
    * If [`RpcSession`] is already reconnecting then a new reconnection
    * attempt won't be performed. Instead, it will wait for the first
    * reconnection attempt result and use it.
    *
    * # Errors
    *
    * With a [`RpcClientException`] if reconnecting attempt fails.
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`RpcClientException`]: crate::api::err::RpcClientException
    * [`RpcSession`]: rpc::RpcSession
    * [`StateError`]: crate::api::err::StateError
    * @param {number} delay_ms
    * @returns {Promise<any>}
    */
    reconnect_with_delay(delay_ms) {
        const ret = wasm.reconnecthandle_reconnect_with_delay(this.ptr, delay_ms);
        return takeObject(ret);
    }
    /**
    * Tries to reconnect a [`RpcSession`] in a loop with a growing backoff
    * delay.
    *
    * The first attempt will be performed immediately, and the second attempt
    * will be performed after `starting_delay_ms`.
    *
    * Delay between reconnection attempts won't be greater than
    * `max_delay_ms`.
    *
    * After each reconnection attempt, delay between reconnections will be
    * multiplied by the given `multiplier` until it reaches `max_delay_ms`.
    *
    *
    * If `multiplier` is a negative number then it will be considered as
    * `0.0`. This might cause a busy loop, so it's not recommended.
    *
    * Max elapsed time can be limited with an optional `max_elapsed_time_ms`
    * argument.
    *
    * If [`RpcSession`] is already reconnecting then new reconnection attempt
    * won't be performed. Instead, it will wait for the first reconnection
    * attempt result and use it here.
    *
    * # Errors
    *
    * With a [`RpcClientException`] if reconnecting attempt fails.
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`RpcClientException`]: crate::api::err::RpcClientException
    * [`RpcSession`]: rpc::RpcSession
    * [`StateError`]: crate::api::err::StateError
    * @param {number} starting_delay_ms
    * @param {number} multiplier
    * @param {number} max_delay
    * @param {number | undefined} max_elapsed_time_ms
    * @returns {Promise<any>}
    */
    reconnect_with_backoff(starting_delay_ms, multiplier, max_delay, max_elapsed_time_ms) {
        const ret = wasm.reconnecthandle_reconnect_with_backoff(this.ptr, starting_delay_ms, multiplier, max_delay, !isLikeNone(max_elapsed_time_ms), isLikeNone(max_elapsed_time_ms) ? 0 : max_elapsed_time_ms);
        return takeObject(ret);
    }
}
/**
* Wrapper around a received remote [MediaStreamTrack][1].
*
* [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
*/
export class RemoteMediaTrack {

    static __wrap(ptr) {
        const obj = Object.create(RemoteMediaTrack.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_remotemediatrack_free(ptr);
    }
    /**
    * Returns the underlying [MediaStreamTrack][1].
    *
    * [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
    * @returns {MediaStreamTrack}
    */
    get_track() {
        const ret = wasm.remotemediatrack_get_track(this.ptr);
        return takeObject(ret);
    }
    /**
    * Indicates whether this [`RemoteMediaTrack`] is muted.
    * @returns {boolean}
    */
    muted() {
        const ret = wasm.remotemediatrack_muted(this.ptr);
        return ret !== 0;
    }
    /**
    * Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
    * @param {Function} cb
    */
    on_muted(cb) {
        wasm.remotemediatrack_on_muted(this.ptr, addHeapObject(cb));
    }
    /**
    * Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
    * @param {Function} cb
    */
    on_unmuted(cb) {
        wasm.remotemediatrack_on_unmuted(this.ptr, addHeapObject(cb));
    }
    /**
    * Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
    * @param {Function} cb
    */
    on_stopped(cb) {
        wasm.remotemediatrack_on_stopped(this.ptr, addHeapObject(cb));
    }
    /**
    * Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
    * [`MediaDirection`] changes.
    * @param {Function} cb
    */
    on_media_direction_changed(cb) {
        wasm.remotemediatrack_on_media_direction_changed(this.ptr, addHeapObject(cb));
    }
    /**
    * Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
    * an audio track, or a [`MediaKind::Video`] if it represents a video
    * track.
    * @returns {number}
    */
    kind() {
        const ret = wasm.remotemediatrack_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns a [`MediaSourceKind::Device`] if this [`RemoteMediaTrack`] is
    * sourced from some device (webcam/microphone), or a
    * [`MediaSourceKind::Display`] if it's captured via
    * [MediaDevices.getDisplayMedia()][1].
    *
    * [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    * @returns {number}
    */
    media_source_kind() {
        const ret = wasm.remotemediatrack_media_source_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns the current general [`MediaDirection`] of this
    * [`RemoteMediaTrack`].
    * @returns {number}
    */
    media_direction() {
        const ret = wasm.remotemediatrack_media_direction(this.ptr);
        return ret >>> 0;
    }
}
/**
* Reason of why a [`Room`] is closed.
*
* This struct is passed to a [`RoomHandle::on_close`] JS side callback.
*
* [`Room`]: room::Room
* [`RoomHandle::on_close`]: crate::api::RoomHandle::on_close
*/
export class RoomCloseReason {

    static __wrap(ptr) {
        const obj = Object.create(RoomCloseReason.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_roomclosereason_free(ptr);
    }
    /**
    * Returns the [`Room`]'s close reason.
    *
    * [`Room`]: room::Room
    * @returns {string}
    */
    reason() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomclosereason_reason(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Indicates whether the [`Room`] was closed by server.
    *
    * [`Room`]: room::Room
    * @returns {boolean}
    */
    is_closed_by_server() {
        const ret = wasm.roomclosereason_is_closed_by_server(this.ptr);
        return ret !== 0;
    }
    /**
    * Indicates whether the [`Room`] close reason is considered as an error.
    *
    * [`Room`]: room::Room
    * @returns {boolean}
    */
    is_err() {
        const ret = wasm.roomclosereason_is_err(this.ptr);
        return ret !== 0;
    }
}
/**
* JS side handle to a [`Room`] where all the media happens.
*
* Like all handles it contains a weak reference to the object that is managed
* by Rust, so its methods will fail if a weak reference could not be upgraded.
*
* [`Room`]: room::Room
*/
export class RoomHandle {

    static __wrap(ptr) {
        const obj = Object.create(RoomHandle.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_roomhandle_free(ptr);
    }
    /**
    * Connects to a media server and joins a [`Room`] with the provided
    * authorization `token`.
    *
    * Authorization token has a fixed format:
    * `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
    * (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
    *
    * Establishes connection with media server (if it doesn't exist already).
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed, or if
    * some mandatory callback is not set. These callbacks are:
    * [`RoomHandle::on_connection_loss`] and
    * [`RoomHandle::on_failed_local_media`].
    *
    * With a [`FormatException`] if the provided `token` string has bad
    * format.
    *
    * With a [`RpcClientException`] if could not connect to a media server.
    *
    * [`FormatException`]: crate::api::err::FormatException
    * [`Room`]: room::Room
    * [`RpcClientException`]: crate::api::err::RpcClientException
    * [`StateError`]: crate::api::err::StateError
    * @param {string} token
    * @returns {Promise<any>}
    */
    join(token) {
        const ptr0 = passStringToWasm0(token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.roomhandle_join(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * Sets callback, invoked when a new [`Connection`] with some remote
    * `Member` is established.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`Connection`]: crate::connection::Connection
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_new_connection(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomhandle_on_new_connection(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets `on_close` callback, invoked when this [`Room`] is closed,
    * providing a [`RoomCloseReason`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`Room`]: room::Room
    * [`RoomCloseReason`]: room::RoomCloseReason
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_close(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomhandle_on_close(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets callback, invoked when a new [`LocalMediaTrack`] is added to this
    * [`Room`].
    *
    * This might happen in such cases:
    * 1. Media server initiates a media request.
    * 2. `enable_audio`/`enable_video` is called.
    * 3. [`MediaStreamSettings`] is updated via `set_local_media_settings`.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`Room`]: room::Room
    * [`LocalMediaTrack`]: crate::api::LocalMediaTrack
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_local_track(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomhandle_on_local_track(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets `on_failed_local_media` callback, invoked on local media
    * acquisition failures.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_failed_local_media(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomhandle_on_failed_local_media(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets `on_connection_loss` callback, invoked when a connection with a
    * server is lost.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * [`StateError`]: crate::api::err::StateError
    * @param {Function} cb
    */
    on_connection_loss(cb) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.roomhandle_on_connection_loss(retptr, this.ptr, addHeapObject(cb));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Updates this [`Room`]s [`MediaStreamSettings`]. This affects all
    * [`PeerConnection`]s in this [`Room`]. If [`MediaStreamSettings`] is
    * configured for some [`Room`], then this [`Room`] can only send media
    * tracks that correspond to this settings. [`MediaStreamSettings`]
    * update will change media tracks in all sending peers, so that might
    * cause new [getUserMedia()][1] request.
    *
    * Media obtaining/injection errors are additionally fired to
    * `on_failed_local_media` callback.
    *
    * If `stop_first` set to `true` then affected [`LocalMediaTrack`]s will be
    * dropped before new [`MediaStreamSettings`] is applied. This is usually
    * required when changing video source device due to hardware limitations,
    * e.g. having an active track sourced from device `A` may hinder
    * [getUserMedia()][1] requests to device `B`.
    *
    * `rollback_on_fail` option configures [`MediaStreamSettings`] update
    * request to automatically rollback to previous settings if new settings
    * cannot be applied.
    *
    * If recovering from fail state isn't possible then affected media types
    * will be disabled.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaSettingsUpdateException`][0] if media settings could not
    * be updated.
    *
    * [`LocalMediaTrack`]: crate::api::LocalMediaTrack
    * [`PeerConnection`]: crate::peer::PeerConnection
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaSettingsUpdateException
    * [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    * @param {MediaStreamSettings} settings
    * @param {boolean} stop_first
    * @param {boolean} rollback_on_fail
    * @returns {Promise<any>}
    */
    set_local_media_settings(settings, stop_first, rollback_on_fail) {
        _assertClass(settings, MediaStreamSettings);
        const ret = wasm.roomhandle_set_local_media_settings(this.ptr, settings.ptr, stop_first, rollback_on_fail);
        return takeObject(ret);
    }
    /**
    * Mutes outbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::unmute_audio()`] was called while muting or a media server
    * didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    mute_audio() {
        const ret = wasm.roomhandle_mute_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Unmutes outbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::mute_audio()`] was called while unmuting or a media server
    * didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    unmute_audio() {
        const ret = wasm.roomhandle_unmute_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Mutes outbound video in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::unmute_video()`] was called while muting or a media server
    * didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    mute_video(source_kind) {
        const ret = wasm.roomhandle_mute_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Unmutes outbound video in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::mute_video()`] was called while unmuting or a media server
    * didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    unmute_video(source_kind) {
        const ret = wasm.roomhandle_unmute_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Disables outbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::enable_audio()`] was called while disabling or a media
    * server didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    disable_audio() {
        const ret = wasm.roomhandle_disable_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Enables outbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::disable_audio()`] was called while enabling or a media
    * server didn't approve this state transition.
    *
    * With a [`LocalMediaInitException`] if a request of platform media
    * devices access failed.
    *
    * [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    enable_audio() {
        const ret = wasm.roomhandle_enable_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Disables outbound video.
    *
    * Affects only video with a specific [`MediaSourceKind`] if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::enable_video()`] was called while disabling or a media
    * server didn't approve this state transition.
    *
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    disable_video(source_kind) {
        const ret = wasm.roomhandle_disable_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Enables outbound video.
    *
    * Affects only video with a specific [`MediaSourceKind`] if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::disable_video()`] was called while enabling or a media
    * server didn't approve this state transition.
    *
    * With a [`LocalMediaInitException`] if a request of platform media
    * devices access failed.
    *
    * [`LocalMediaInitException`]: crate::api::err::LocalMediaInitException
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    enable_video(source_kind) {
        const ret = wasm.roomhandle_enable_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Disables inbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::enable_remote_audio()`] was called while disabling or a
    * media server didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    disable_remote_audio() {
        const ret = wasm.roomhandle_disable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Disables inbound video in this [`Room`].
    *
    * Affects only video with the specific [`MediaSourceKind`], if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::enable_remote_video()`] was called while disabling or a
    * media server didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    disable_remote_video(source_kind) {
        const ret = wasm.roomhandle_disable_remote_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
    /**
    * Enables inbound audio in this [`Room`].
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::disable_remote_audio()`] was called while enabling or a
    * media server didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @returns {Promise<any>}
    */
    enable_remote_audio() {
        const ret = wasm.roomhandle_enable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Enables inbound video in this [`Room`].
    *
    * Affects only video with the specific [`MediaSourceKind`], if specified.
    *
    * # Errors
    *
    * With a [`StateError`] if the underlying pointer has been freed.
    *
    * With a [`MediaStateTransitionException`][0] if
    * [`RoomHandle::disable_remote_video()`] was called while enabling or a
    * media server didn't approve this state transition.
    *
    * [`Room`]: room::Room
    * [`StateError`]: crate::api::err::StateError
    * [0]: crate::api::err::MediaStateTransitionException
    * @param {number | undefined} source_kind
    * @returns {Promise<any>}
    */
    enable_remote_video(source_kind) {
        const ret = wasm.roomhandle_enable_remote_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return takeObject(ret);
    }
}
/**
* Exceptions thrown from a RPC client that implements messaging with media
* server.
*/
export class RpcClientException {

    static __wrap(ptr) {
        const obj = Object.create(RpcClientException.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rpcclientexception_free(ptr);
    }
    /**
    * Returns concrete error kind of this [`RpcClientException`].
    * @returns {number}
    */
    kind() {
        const ret = wasm.rpcclientexception_kind(this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns an error message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns [`platform::Error`] causing this [`RpcClientException`].
    * @returns {Error | undefined}
    */
    cause() {
        const ret = wasm.internalexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns stacktrace of this [`RpcClientException`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.internalexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Error thrown when the operation wasn't allowed by the current state of the
* object.
*/
export class StateError {

    static __wrap(ptr) {
        const obj = Object.create(StateError.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stateerror_free(ptr);
    }
    /**
    * Returns message describing the problem.
    * @returns {string}
    */
    message() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.formatexception_message(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns native stacktrace of this [`StateError`].
    * @returns {string}
    */
    trace() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mediastatetransitionexception_trace(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_json_serialize = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = JSON.stringify(obj === undefined ? null : obj);
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_stateerror_new = function(arg0) {
        const ret = StateError.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_enumeratedevicesexception_new = function(arg0) {
        const ret = EnumerateDevicesException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_localmediainitexception_new = function(arg0) {
        const ret = LocalMediaInitException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_rpcclientexception_new = function(arg0) {
        const ret = RpcClientException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_internalexception_new = function(arg0) {
        const ret = InternalException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_formatexception_new = function(arg0) {
        const ret = FormatException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediastatetransitionexception_new = function(arg0) {
        const ret = MediaStateTransitionException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediasettingsupdateexception_new = function(arg0) {
        const ret = MediaSettingsUpdateException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_reconnecthandle_new = function(arg0) {
        const ret = ReconnectHandle.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_connectionhandle_new = function(arg0) {
        const ret = ConnectionHandle.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_roomclosereason_new = function(arg0) {
        const ret = RoomCloseReason.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_localmediatrack_new = function(arg0) {
        const ret = LocalMediaTrack.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_remotemediatrack_new = function(arg0) {
        const ret = RemoteMediaTrack.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediadeviceinfo_new = function(arg0) {
        const ret = MediaDeviceInfo.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_randomFillSync_91e2b39becca6147 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_b14734aa289bc356 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_process_e56fd54cf6319b6c = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_versions_77e21455908dad33 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_0dd25d832e4785d5 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd = function() {
        const ret = module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_0db1598d9ccecb30 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_crypto_b95d7173266618a9 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_5a86d77a66230f81 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_a2a08d3918d7d4d0 = function(arg0) {
        const ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_navigator_2d05aef684d827d8 = function(arg0) {
        const ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_7d328aba48109be9 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_debug_8445d9e6649f346e = function(arg0, arg1, arg2, arg3) {
        console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_error_e2677af4c7f31a14 = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_error_c67ca41b33779f06 = function(arg0, arg1, arg2, arg3) {
        console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_info_89e9ec243332464d = function(arg0, arg1, arg2, arg3) {
        console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_log_4a85132d4d89d41e = function(arg0, arg1, arg2, arg3) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_warn_022e4f61a9c7964f = function(arg0, arg1, arg2, arg3) {
        console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_code_790e295d8679c12a = function(arg0) {
        const ret = getObject(arg0).code;
        return ret;
    };
    imports.wbg.__wbg_reason_37f66f7f8b4f5c3c = function(arg0, arg1) {
        const ret = getObject(arg1).reason;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_replaceTrack_7dbd459c2d372279 = function(arg0, arg1) {
        const ret = getObject(arg0).replaceTrack(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_now_9c64828adecad05e = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_data_751f064cdd700ef0 = function(arg0) {
        const ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_enumerateDevices_869594c673a77400 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).enumerateDevices();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getDisplayMedia_29c33ff7026398bd = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).getDisplayMedia(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getUserMedia_036d4786d181f856 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).getUserMedia(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_track_4aee9c52b9bc62f5 = function(arg0) {
        const ret = getObject(arg0).track;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_transceiver_2b58652d77fb2e42 = function(arg0) {
        const ret = getObject(arg0).transceiver;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_candidate_5486c2381f233ffd = function(arg0, arg1) {
        const ret = getObject(arg1).candidate;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sdpMid_b299ff28e82355b1 = function(arg0, arg1) {
        const ret = getObject(arg1).sdpMid;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sdpMLineIndex_d8cabf36d274d09e = function(arg0) {
        const ret = getObject(arg0).sdpMLineIndex;
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_candidate_0944f4ed58d5b077 = function(arg0) {
        const ret = getObject(arg0).candidate;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_iceConnectionState_733aba3416db0499 = function(arg0) {
        const ret = getObject(arg0).iceConnectionState;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithconfiguration_47385916076404e5 = function() { return handleError(function (arg0) {
        const ret = new RTCPeerConnection(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_addIceCandidate_dfa8a531b6b20a1f = function(arg0, arg1) {
        const ret = getObject(arg0).addIceCandidate(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_addTransceiver_50e91d0bd9fa0e21 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).addTransceiver(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_close_47d71e1a34ef6257 = function(arg0) {
        getObject(arg0).close();
    };
    imports.wbg.__wbg_createAnswer_01879bf0b7ddfc18 = function(arg0) {
        const ret = getObject(arg0).createAnswer();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createOffer_409a9463cdd483f3 = function(arg0, arg1) {
        const ret = getObject(arg0).createOffer(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getStats_9ba27351ec2e51e0 = function(arg0) {
        const ret = getObject(arg0).getStats();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getTransceivers_96577ebc19b6cd8d = function(arg0) {
        const ret = getObject(arg0).getTransceivers();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setLocalDescription_04bce5f8eaea39ec = function(arg0, arg1) {
        const ret = getObject(arg0).setLocalDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setRemoteDescription_c563af7a67387483 = function(arg0, arg1) {
        const ret = getObject(arg0).setRemoteDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_kind_fa65f54ba718961d = function(arg0, arg1) {
        const ret = getObject(arg1).kind;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_id_b2f165a43ee0df7d = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setenabled_eef1d553092c47be = function(arg0, arg1) {
        getObject(arg0).enabled = arg1 !== 0;
    };
    imports.wbg.__wbg_readyState_fcd40e65cc6746f9 = function(arg0) {
        const ret = getObject(arg0).readyState;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_clone_1c97c5741905df93 = function(arg0) {
        const ret = getObject(arg0).clone();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getSettings_3615b15efebc070e = function(arg0) {
        const ret = getObject(arg0).getSettings();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stop_f7a152465288dca4 = function(arg0) {
        getObject(arg0).stop();
    };
    imports.wbg.__wbg_mid_4eb8b88d6730b3b2 = function(arg0, arg1) {
        const ret = getObject(arg1).mid;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sender_4b81dce465c16935 = function(arg0) {
        const ret = getObject(arg0).sender;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stopped_47a20ea4104844cb = function(arg0) {
        const ret = getObject(arg0).stopped;
        return ret;
    };
    imports.wbg.__wbg_direction_74f5ac57c78299df = function(arg0) {
        const ret = getObject(arg0).direction;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setdirection_0e5b4dc76154d053 = function(arg0, arg1) {
        getObject(arg0).direction = takeObject(arg1);
    };
    imports.wbg.__wbg_sdp_65e4812f0379adad = function(arg0, arg1) {
        const ret = getObject(arg1).sdp;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_mediaDevices_a8a0411e8ad1279c = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).mediaDevices;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getTracks_8279f01fb86d6781 = function(arg0) {
        const ret = getObject(arg0).getTracks();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_addEventListener_5822223857fe82cb = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_0e2fd54517fc188b = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_new_6553602c5dd43c85 = function() { return handleError(function (arg0, arg1) {
        const ret = new WebSocket(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_2f422b25680395f3 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).close(arg1, getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_send_78d653e63982c7dc = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_deviceId_c1700ac63e2f6d24 = function(arg0, arg1) {
        const ret = getObject(arg1).deviceId;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_kind_0d7a31b469e0a853 = function(arg0) {
        const ret = getObject(arg0).kind;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_label_9818e55e39361da4 = function(arg0, arg1) {
        const ret = getObject(arg1).label;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_groupId_dab4bc84e03894dc = function(arg0, arg1) {
        const ret = getObject(arg1).groupId;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_get_f0f4f1608ebf633e = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_2ab697f1555e0dbc = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_newnoargs_fc5356289219b93b = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_0e1ee6203bc0f8ed = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_9ef803116340cdc1 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_2a1e30464aae6a4d = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_a495c29471c31da6 = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_6ac6eb1e020f18e3 = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_89247d3aeaa38cc5 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_4573f605ca4b5f10 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_306ce8d57919e6ae = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_ba1ddafe9ea7a3a2 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_be3cc430364fd32c = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_56d9c9f814daeeee = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_8c35aeee4ac77f2b = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_from_7ff9036e9b5c3ccb = function(arg0) {
        const ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_811c8b08bf4ff9d5 = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_values_644ed6c9e45c1e38 = function(arg0) {
        const ret = getObject(arg0).values();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Error_53fd3b982f19be06 = function(arg0) {
        const ret = getObject(arg0) instanceof Error;
        return ret;
    };
    imports.wbg.__wbg_new_651776e932b7e9c7 = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_message_136debd54c3edfe4 = function(arg0) {
        const ret = getObject(arg0).message;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_toString_ef76a2af8f5bb98a = function(arg0) {
        const ret = getObject(arg0).toString();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_9855a4612eb496cb = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_is_aafa609b540ad47f = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_78403b138428b684 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_348(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_f269ce174f88b294 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_1c698eedca15eed6 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_4debc41d4fc92ce5 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_de1150f91b23aa89 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_97cf52648830a70d = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_a0172b213e2469e9 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_e09c0b925ab8de5d = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_e833b89f9db02732 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_9482ae5cd5cd99d3 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_b12cd0ab82903c2f = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1114 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 418, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1115 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 418, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1116 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 418, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1120 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 418, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1122 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 418, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2803 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 805, __wbg_adapter_43);
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);


    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('medea_jason_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;

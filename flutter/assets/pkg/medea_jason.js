
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

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

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
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
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12e6cc6bfa2db6b0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12e6cc6bfa2db6b0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_38(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12e6cc6bfa2db6b0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_41(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12e6cc6bfa2db6b0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_44(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h12e6cc6bfa2db6b0(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_47(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3c0d19d6781a276b(arg0, arg1, addHeapObject(arg2));
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
function __wbg_adapter_343(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h506acbb810aa0090(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
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
* Occurs if the [getDisplayMedia()][1] request failed.
*
* [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
*/
GetDisplayMediaFailed:1,"1":"GetDisplayMediaFailed",
/**
* Occurs when local track is [`ended`][1] right after [getUserMedia()][2]
* or [getDisplayMedia()][3] request.
*
* [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
* [2]: https://tinyurl.com/rnxcavf
* [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
*/
LocalTrackIsEnded:2,"2":"LocalTrackIsEnded", });
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
* [1]: https://www.w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-kind
*/
export const MediaDeviceKind = Object.freeze({
/**
* Represents an audio input device; for example a microphone.
*/
AudioInput:0,"0":"AudioInput",
/**
* Represents a video input device; for example a webcam.
*/
VideoInput:1,"1":"VideoInput",
/**
* Represents an audio output device; for example a pair of headphones.
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
        var ret = wasm.audiotrackconstraints_new();
        return AudioTrackConstraints.__wrap(ret);
    }
    /**
    * Sets an exact [deviceId][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    * @param {string} device_id
    */
    device_id(device_id) {
        var ptr0 = passStringToWasm0(device_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
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
        var ret = wasm.devicevideotrackconstraints_new();
        return DeviceVideoTrackConstraints.__wrap(ret);
    }
    /**
    * Sets an exact [deviceId][1] constraint.
    *
    * [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    * @param {string} device_id
    */
    device_id(device_id) {
        var ptr0 = passStringToWasm0(device_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
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
        var ret = wasm.displayvideotrackconstraints_new();
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
        var ret = wasm.enumeratedevicesexception_cause(this.ptr);
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
        var ret = wasm.internalexception_cause(this.ptr);
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
        var ret = wasm.jason_new();
        return Jason.__wrap(ret);
    }
    /**
    * Creates a new `Room` and returns its [`RoomHandle`].
    * @returns {RoomHandle}
    */
    init_room() {
        var ret = wasm.jason_init_room(this.ptr);
        return RoomHandle.__wrap(ret);
    }
    /**
    * Returns a [`MediaManagerHandle`].
    * @returns {MediaManagerHandle}
    */
    media_manager() {
        var ret = wasm.jason_media_manager(this.ptr);
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
        var ret = wasm.localmediainitexception_kind(this.ptr);
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
        var ret = wasm.internalexception_cause(this.ptr);
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
        var ret = wasm.localmediatrack_get_track(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
    * audio track, or a [`MediaKind::Video`] if it represents a video track.
    * @returns {number}
    */
    kind() {
        var ret = wasm.localmediatrack_kind(this.ptr);
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
        var ret = wasm.localmediatrack_media_source_kind(this.ptr);
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
        var ret = wasm.mediadeviceinfo_kind(this.ptr);
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
        var ret = wasm.mediamanagerhandle_enumerate_devices(this.ptr);
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
        var ret = wasm.mediamanagerhandle_init_local_tracks(this.ptr, caps.ptr);
        return takeObject(ret);
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
    * Returns the original [`ChangeMediaStateError`] that was encountered
    * while updating local media settings.
    * @returns {any}
    */
    cause() {
        var ret = wasm.mediasettingsupdateexception_cause(this.ptr);
        return takeObject(ret);
    }
    /**
    * Returns whether media settings were successfully rolled back after new
    * settings application failed.
    * @returns {boolean}
    */
    rolled_back() {
        var ret = wasm.mediasettingsupdateexception_rolled_back(this.ptr);
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
        var ret = wasm.mediastreamsettings_new();
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
        var ret = wasm.reconnecthandle_reconnect_with_delay(this.ptr, delay_ms);
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
        var ret = wasm.reconnecthandle_reconnect_with_backoff(this.ptr, starting_delay_ms, multiplier, max_delay, !isLikeNone(max_elapsed_time_ms), isLikeNone(max_elapsed_time_ms) ? 0 : max_elapsed_time_ms);
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
        var ret = wasm.remotemediatrack_get_track(this.ptr);
        return takeObject(ret);
    }
    /**
    * Indicates whether this [`RemoteMediaTrack`] is enabled.
    * @returns {boolean}
    */
    enabled() {
        var ret = wasm.remotemediatrack_enabled(this.ptr);
        return ret !== 0;
    }
    /**
    * Indicates whether this [`RemoteMediaTrack`] is muted.
    * @returns {boolean}
    */
    muted() {
        var ret = wasm.remotemediatrack_muted(this.ptr);
        return ret !== 0;
    }
    /**
    * Sets callback, invoked when this [`RemoteMediaTrack`] is enabled.
    * @param {Function} cb
    */
    on_enabled(cb) {
        wasm.remotemediatrack_on_enabled(this.ptr, addHeapObject(cb));
    }
    /**
    * Sets callback, invoked when this [`RemoteMediaTrack`] is disabled.
    * @param {Function} cb
    */
    on_disabled(cb) {
        wasm.remotemediatrack_on_disabled(this.ptr, addHeapObject(cb));
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
    * Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
    * an audio track, or a [`MediaKind::Video`] if it represents a video
    * track.
    * @returns {number}
    */
    kind() {
        var ret = wasm.remotemediatrack_kind(this.ptr);
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
        var ret = wasm.remotemediatrack_media_source_kind(this.ptr);
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
        var ret = wasm.roomclosereason_is_closed_by_server(this.ptr);
        return ret !== 0;
    }
    /**
    * Indicates whether the [`Room`] close reason is considered as an error.
    *
    * [`Room`]: room::Room
    * @returns {boolean}
    */
    is_err() {
        var ret = wasm.roomclosereason_is_err(this.ptr);
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
        var ptr0 = passStringToWasm0(token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.roomhandle_join(this.ptr, ptr0, len0);
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
        var ret = wasm.roomhandle_set_local_media_settings(this.ptr, settings.ptr, stop_first, rollback_on_fail);
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
        var ret = wasm.roomhandle_mute_audio(this.ptr);
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
        var ret = wasm.roomhandle_unmute_audio(this.ptr);
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
        var ret = wasm.roomhandle_mute_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
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
        var ret = wasm.roomhandle_unmute_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
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
        var ret = wasm.roomhandle_disable_audio(this.ptr);
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
        var ret = wasm.roomhandle_enable_audio(this.ptr);
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
        var ret = wasm.roomhandle_disable_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
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
        var ret = wasm.roomhandle_enable_video(this.ptr, isLikeNone(source_kind) ? 2 : source_kind);
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
        var ret = wasm.roomhandle_disable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Disables inbound video in this [`Room`].
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
    * @returns {Promise<any>}
    */
    disable_remote_video() {
        var ret = wasm.roomhandle_disable_remote_video(this.ptr);
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
        var ret = wasm.roomhandle_enable_remote_audio(this.ptr);
        return takeObject(ret);
    }
    /**
    * Enables inbound video in this [`Room`].
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
    * @returns {Promise<any>}
    */
    enable_remote_video() {
        var ret = wasm.roomhandle_enable_remote_video(this.ptr);
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
        var ret = wasm.localmediainitexception_kind(this.ptr);
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
        var ret = wasm.internalexception_cause(this.ptr);
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

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('medea_jason_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_localmediatrack_new = function(arg0) {
        var ret = LocalMediaTrack.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_json_serialize = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = JSON.stringify(obj === undefined ? null : obj);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        var ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stateerror_new = function(arg0) {
        var ret = StateError.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_localmediainitexception_new = function(arg0) {
        var ret = LocalMediaInitException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_enumeratedevicesexception_new = function(arg0) {
        var ret = EnumerateDevicesException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_rpcclientexception_new = function(arg0) {
        var ret = RpcClientException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_internalexception_new = function(arg0) {
        var ret = InternalException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_formatexception_new = function(arg0) {
        var ret = FormatException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediastatetransitionexception_new = function(arg0) {
        var ret = MediaStateTransitionException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediasettingsupdateexception_new = function(arg0) {
        var ret = MediaSettingsUpdateException.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_roomclosereason_new = function(arg0) {
        var ret = RoomCloseReason.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_remotemediatrack_new = function(arg0) {
        var ret = RemoteMediaTrack.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_reconnecthandle_new = function(arg0) {
        var ret = ReconnectHandle.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_mediadeviceinfo_new = function(arg0) {
        var ret = MediaDeviceInfo.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_connectionhandle_new = function(arg0) {
        var ret = ConnectionHandle.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
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
    imports.wbg.__wbg_randomFillSync_378e02b85af41ab6 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_99bbe8a65f4aef87 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_process_5729605ce9d34ea8 = function(arg0) {
        var ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        var ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_versions_531e16e1a776ee97 = function(arg0) {
        var ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_18b58a160b60d170 = function(arg0) {
        var ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb = function() {
        var ret = module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_edfaedd93e302925 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_crypto_2bc4d5b05161de5b = function(arg0) {
        var ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_d003eebe62c636a9 = function(arg0) {
        var ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_navigator_0e0588c949560476 = function(arg0) {
        var ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_1c75092906446b91 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_debug_6df4b1a327dd2e94 = function(arg0, arg1, arg2, arg3) {
        console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_error_ca520cb687b085a1 = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_error_644d3bc8c0537e80 = function(arg0, arg1, arg2, arg3) {
        console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_info_8bed0988e7416289 = function(arg0, arg1, arg2, arg3) {
        console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_log_681299aef22afa27 = function(arg0, arg1, arg2, arg3) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_warn_ca021eeadd0df9cd = function(arg0, arg1, arg2, arg3) {
        console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    };
    imports.wbg.__wbg_code_32a97f32b2304d71 = function(arg0) {
        var ret = getObject(arg0).code;
        return ret;
    };
    imports.wbg.__wbg_reason_51cb1f322946c0a8 = function(arg0, arg1) {
        var ret = getObject(arg1).reason;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_replaceTrack_1b05265193c931e5 = function(arg0, arg1) {
        var ret = getObject(arg0).replaceTrack(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_now_5fa0ca001e042f8a = function(arg0) {
        var ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_data_44aaea098b9f4c6b = function(arg0) {
        var ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_enumerateDevices_dd9d2792dc481184 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).enumerateDevices();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getDisplayMedia_df053cf5443424c5 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).getDisplayMedia(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getUserMedia_55aa2788a18b9774 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).getUserMedia(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_track_4ccdf1baaba23079 = function(arg0) {
        var ret = getObject(arg0).track;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_transceiver_b1f6f687e323aa2b = function(arg0) {
        var ret = getObject(arg0).transceiver;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_candidate_f4af4aaedf706289 = function(arg0, arg1) {
        var ret = getObject(arg1).candidate;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sdpMid_9318f9d860ed4841 = function(arg0, arg1) {
        var ret = getObject(arg1).sdpMid;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sdpMLineIndex_fb37ae5073ed3f68 = function(arg0) {
        var ret = getObject(arg0).sdpMLineIndex;
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_candidate_0c0b5e21bdceb48e = function(arg0) {
        var ret = getObject(arg0).candidate;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_iceConnectionState_925970df8128ea7d = function(arg0) {
        var ret = getObject(arg0).iceConnectionState;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithconfiguration_fdd8e086bd06b548 = function() { return handleError(function (arg0) {
        var ret = new RTCPeerConnection(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_addIceCandidate_189b0ddb4bda9265 = function(arg0, arg1) {
        var ret = getObject(arg0).addIceCandidate(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_addTransceiver_a5dcd1dfd8703ba2 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).addTransceiver(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_close_e022b407c0bf6a74 = function(arg0) {
        getObject(arg0).close();
    };
    imports.wbg.__wbg_createAnswer_c0cb49912efe992a = function(arg0) {
        var ret = getObject(arg0).createAnswer();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createOffer_4ca6c4468e5aa9b0 = function(arg0, arg1) {
        var ret = getObject(arg0).createOffer(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getStats_b8d8cb0371cf5e75 = function(arg0) {
        var ret = getObject(arg0).getStats();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getTransceivers_2d591fea77ede751 = function(arg0) {
        var ret = getObject(arg0).getTransceivers();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setLocalDescription_7063a95d5b0f6d3a = function(arg0, arg1) {
        var ret = getObject(arg0).setLocalDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setRemoteDescription_627f057fec1b43ab = function(arg0, arg1) {
        var ret = getObject(arg0).setRemoteDescription(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_kind_2675ed94f8d218d0 = function(arg0, arg1) {
        var ret = getObject(arg1).kind;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_id_4e75a22c36e1059f = function(arg0, arg1) {
        var ret = getObject(arg1).id;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setenabled_2ecb88f65a56a93d = function(arg0, arg1) {
        getObject(arg0).enabled = arg1 !== 0;
    };
    imports.wbg.__wbg_readyState_c5f5945b31c24284 = function(arg0) {
        var ret = getObject(arg0).readyState;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_clone_2db71c8c551ce4f0 = function(arg0) {
        var ret = getObject(arg0).clone();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getSettings_1c266d3cf8287c34 = function(arg0) {
        var ret = getObject(arg0).getSettings();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stop_193f790c38f39d77 = function(arg0) {
        getObject(arg0).stop();
    };
    imports.wbg.__wbg_mid_fea259559e349485 = function(arg0, arg1) {
        var ret = getObject(arg1).mid;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_sender_0d519e09749ec40b = function(arg0) {
        var ret = getObject(arg0).sender;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stopped_fccb7b580cdd115e = function(arg0) {
        var ret = getObject(arg0).stopped;
        return ret;
    };
    imports.wbg.__wbg_direction_ae94b70b1cbadd1d = function(arg0) {
        var ret = getObject(arg0).direction;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setdirection_4f3b55edf4c1bb41 = function(arg0, arg1) {
        getObject(arg0).direction = takeObject(arg1);
    };
    imports.wbg.__wbg_sdp_cfdb4ca5db1f89f8 = function(arg0, arg1) {
        var ret = getObject(arg1).sdp;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_mediaDevices_931f35a83dbb1470 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).mediaDevices;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getTracks_ba7ed3118f55bfb4 = function(arg0) {
        var ret = getObject(arg0).getTracks();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_9d38005ad72b669a = function() { return handleError(function (arg0, arg1) {
        var ret = new WebSocket(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_a9201eb665c13af3 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).close(arg1, getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_send_2bad75269a8cc966 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).send(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_addEventListener_6bdba88519fdc1c9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_8d16089e686f486a = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_deviceId_b2f8c2009a30d737 = function(arg0, arg1) {
        var ret = getObject(arg1).deviceId;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_kind_9192ec36f0ed987a = function(arg0) {
        var ret = getObject(arg0).kind;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_label_fa1157aba8d05487 = function(arg0, arg1) {
        var ret = getObject(arg1).label;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_groupId_1c50c0815e1d49b0 = function(arg0, arg1) {
        var ret = getObject(arg1).groupId;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_get_f45dff51f52d7222 = function(arg0, arg1) {
        var ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_16f24b0728c5e67b = function() {
        var ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_newnoargs_f579424187aa1717 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_c7a2a6b012059a5e = function(arg0) {
        var ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_dd1a890d37e38d73 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_982b1c7ac0cbc69d = function(arg0) {
        var ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_2def2d1fb38b02cd = function(arg0) {
        var ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_4b9cedbeda0c0e30 = function() {
        var ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_8bbb82393651dd9c = function() { return handleError(function (arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_d3138911a89329b0 = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_from_4216160a11e086ef = function(arg0) {
        var ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_a72df856079e6930 = function(arg0, arg1) {
        var ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_values_71935f80778b5113 = function(arg0) {
        var ret = getObject(arg0).values();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Error_4287ce7d75f0e3a2 = function(arg0) {
        var ret = getObject(arg0) instanceof Error;
        return ret;
    };
    imports.wbg.__wbg_new_55259b13834a484c = function(arg0, arg1) {
        var ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_toString_3e854a6a919f2996 = function(arg0) {
        var ret = getObject(arg0).toString();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_94697a95cb7e239c = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_is_3d73f4d91adacc37 = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_4beacc9c71572250 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_343(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            var ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_4f8f547f26b30b27 = function(arg0) {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_a6860c82b90816ca = function(arg0, arg1) {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_58a04e42527f52c6 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_e23d74ae45fb17d1 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_b4be7f48b24ac56e = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_d61b1f48a57191ae = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_e7669da72fd7f239 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_buffer_5e74a88a1424a2e0 = function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_e3b800e570795b3c = function(arg0) {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_5b8081e9d002f0df = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_30803400a8f15c59 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_5f4ce114a24dfe1e = function(arg0) {
        var ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_a68f835ca2af506f = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_c42875065132a932 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2640 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 710, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2641 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 710, __wbg_adapter_35);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2642 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 710, __wbg_adapter_38);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2643 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 710, __wbg_adapter_41);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2651 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 710, __wbg_adapter_44);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2748 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 777, __wbg_adapter_47);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;


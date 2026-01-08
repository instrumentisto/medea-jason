const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

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
    if (builtInMatches && builtInMatches.length > 1) {
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

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
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
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

function wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h359418e947c31e42(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h359418e947c31e42(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h0b83e4d6cdffc65d(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0b83e4d6cdffc65d(arg0, arg1, arg2, arg3);
}

const __wbindgen_enum_AudioContextState = ["suspended", "running", "closed"];

const __wbindgen_enum_MediaDeviceKind = ["audioinput", "audiooutput", "videoinput"];

const __wbindgen_enum_MediaStreamTrackState = ["live", "ended"];

const __wbindgen_enum_RtcBundlePolicy = ["balanced", "max-compat", "max-bundle"];

const __wbindgen_enum_RtcIceConnectionState = ["new", "checking", "connected", "completed", "failed", "disconnected", "closed"];

const __wbindgen_enum_RtcIceGatheringState = ["new", "gathering", "complete"];

const __wbindgen_enum_RtcIceTransportPolicy = ["relay", "all"];

const __wbindgen_enum_RtcPeerConnectionState = ["closed", "failed", "disconnected", "new", "connecting", "connected"];

const __wbindgen_enum_RtcRtpTransceiverDirection = ["sendrecv", "sendonly", "recvonly", "inactive", "stopped"];

const __wbindgen_enum_RtcSdpType = ["offer", "pranswer", "answer", "rollback"];

const ConnectionHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_connectionhandle_free(ptr >>> 0, 1));

const DeviceAudioTrackConstraintsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_deviceaudiotrackconstraints_free(ptr >>> 0, 1));

const DeviceVideoTrackConstraintsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_devicevideotrackconstraints_free(ptr >>> 0, 1));

const DisplayAudioTrackConstraintsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_displayaudiotrackconstraints_free(ptr >>> 0, 1));

const DisplayVideoTrackConstraintsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_displayvideotrackconstraints_free(ptr >>> 0, 1));

const EnumerateDevicesExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_enumeratedevicesexception_free(ptr >>> 0, 1));

const FormatExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_formatexception_free(ptr >>> 0, 1));

const InternalExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_internalexception_free(ptr >>> 0, 1));

const InvalidOutputAudioDeviceIdExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_invalidoutputaudiodeviceidexception_free(ptr >>> 0, 1));

const JasonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jason_free(ptr >>> 0, 1));

const LocalMediaInitExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_localmediainitexception_free(ptr >>> 0, 1));

const LocalMediaTrackFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_localmediatrack_free(ptr >>> 0, 1));

const MediaDeviceDetailsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mediadevicedetails_free(ptr >>> 0, 1));

const MediaManagerHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mediamanagerhandle_free(ptr >>> 0, 1));

const MediaSettingsUpdateExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mediasettingsupdateexception_free(ptr >>> 0, 1));

const MediaStateTransitionExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mediastatetransitionexception_free(ptr >>> 0, 1));

const MediaStreamSettingsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mediastreamsettings_free(ptr >>> 0, 1));

const MemberConnectionStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_memberconnectionstate_free(ptr >>> 0, 1));

const MicVolumeExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_micvolumeexception_free(ptr >>> 0, 1));

const ReconnectHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_reconnecthandle_free(ptr >>> 0, 1));

const RemoteMediaTrackFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_remotemediatrack_free(ptr >>> 0, 1));

const RoomCloseReasonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_roomclosereason_free(ptr >>> 0, 1));

const RoomHandleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_roomhandle_free(ptr >>> 0, 1));

const RpcClientExceptionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rpcclientexception_free(ptr >>> 0, 1));

const StateErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stateerror_free(ptr >>> 0, 1));

/**
 * Connection with a specific remote `Member`, that is used on JS side.
 *
 * Like all the handles it contains a weak reference to the object that is
 * managed by Rust, so its methods will fail if a weak reference could not be
 * upgraded.
 */
export class ConnectionHandle {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ConnectionHandle.prototype);
        obj.__wbg_ptr = ptr;
        ConnectionHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ConnectionHandleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_connectionhandle_free(ptr, 0);
    }
    /**
     * Sets a callback to be invoked once a state of the associated
     * [`Connection`] is changed.
     *
     * __NOTE__: Only works in [P2P mesh] mode and is subject to change.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
     * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
     * a [`RoomHandle`] that implicitly owns native object behind this
     * [`ConnectionHandle`].
     *
     * [`Connection`]: connection::Connection
     * [`Jason`]: api::Jason
     * [`RoomHandle`]: api::RoomHandle
     * [`StateError`]: crate::api::err::StateError
     * @param {Function} cb
     */
    on_state_change(cb) {
        const ret = wasm.connectionhandle_on_state_change(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
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
        const ret = wasm.connectionhandle_enable_remote_audio(this.__wbg_ptr);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    enable_remote_video(source_kind) {
        const ret = wasm.connectionhandle_enable_remote_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
        const ret = wasm.connectionhandle_disable_remote_audio(this.__wbg_ptr);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    disable_remote_video(source_kind) {
        const ret = wasm.connectionhandle_disable_remote_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
    }
    /**
     * Returns ID of the remote `Member`.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.connectionhandle_get_remote_member_id(this.__wbg_ptr);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Sets callback, invoked when a new [`RemoteMediaTrack`] is added to this
     * [`Connection`].
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        const ret = wasm.connectionhandle_on_remote_track_added(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Sets callback, invoked when connection quality score is updated by a
     * server.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        const ret = wasm.connectionhandle_on_quality_score_update(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Sets callback, invoked when this [`Connection`] is closed.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        const ret = wasm.connectionhandle_on_close(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Returns the [`MemberConnectionState`] of the associated [`Connection`].
     *
     * __NOTE__: Only works in [P2P mesh] mode and is subject to change.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
     * `free` was called on this [`ConnectionHandle`], or on a [`Jason`], or on
     * a [`RoomHandle`] that implicitly owns native object behind this
     * [`ConnectionHandle`].
     *
     * [`Connection`]: connection::Connection
     * [`Jason`]: api::Jason
     * [`RoomHandle`]: api::RoomHandle
     * [`StateError`]: crate::api::err::StateError
     * [P2P mesh]: https://webrtcglossary.com/mesh
     * @returns {MemberConnectionState | undefined}
     */
    get_state() {
        const ret = wasm.connectionhandle_get_state(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] === 0 ? undefined : MemberConnectionState.__wrap(ret[0]);
    }
}
if (Symbol.dispose) ConnectionHandle.prototype[Symbol.dispose] = ConnectionHandle.prototype.free;

/**
 * Constraints applicable to device audio tracks (microphone).
 */
export class DeviceAudioTrackConstraints {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DeviceAudioTrackConstraintsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_deviceaudiotrackconstraints_free(ptr, 0);
    }
    /**
     * Sets an exact [autoGainControl][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-autogaincontrol
     * @param {boolean} agc
     */
    exact_auto_gain_control(agc) {
        wasm.deviceaudiotrackconstraints_exact_auto_gain_control(this.__wbg_ptr, agc);
    }
    /**
     * Sets an exact [echoCancellation][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-echocancellation
     * @param {boolean} aec
     */
    exact_echo_cancellation(aec) {
        wasm.deviceaudiotrackconstraints_exact_echo_cancellation(this.__wbg_ptr, aec);
    }
    /**
     * Sets an exact [noiseSuppression][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-noisesuppression
     * @param {boolean} ns
     */
    exact_noise_suppression(ns) {
        wasm.deviceaudiotrackconstraints_exact_noise_suppression(this.__wbg_ptr, ns);
    }
    /**
     * Sets an ideal [autoGainControl][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-autogaincontrol
     * @param {boolean} agc
     */
    ideal_auto_gain_control(agc) {
        wasm.deviceaudiotrackconstraints_ideal_auto_gain_control(this.__wbg_ptr, agc);
    }
    /**
     * Sets an ideal [echoCancellation][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-echocancellation
     * @param {boolean} aec
     */
    ideal_echo_cancellation(aec) {
        wasm.deviceaudiotrackconstraints_ideal_echo_cancellation(this.__wbg_ptr, aec);
    }
    /**
     * Sets an ideal [noiseSuppression][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-noisesuppression
     * @param {boolean} ns
     */
    ideal_noise_suppression(ns) {
        wasm.deviceaudiotrackconstraints_ideal_noise_suppression(this.__wbg_ptr, ns);
    }
    /**
     * Creates new [`DeviceAudioTrackConstraints`] with none constraints
     * configured.
     */
    constructor() {
        const ret = wasm.deviceaudiotrackconstraints_new();
        this.__wbg_ptr = ret >>> 0;
        DeviceAudioTrackConstraintsFinalization.register(this, this.__wbg_ptr, this);
        return this;
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
        wasm.deviceaudiotrackconstraints_device_id(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) DeviceAudioTrackConstraints.prototype[Symbol.dispose] = DeviceAudioTrackConstraints.prototype.free;

/**
 * Constraints applicable to audio tracks, sourced from a system audio
 * recording device (usually a microphone).
 */
export class DeviceVideoTrackConstraints {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DeviceVideoTrackConstraintsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_devicevideotrackconstraints_free(ptr, 0);
    }
    /**
     * Sets an exact [`width`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-width
     * @param {number} width
     */
    exact_width(width) {
        wasm.devicevideotrackconstraints_exact_width(this.__wbg_ptr, width);
    }
    /**
     * Sets an ideal [`width`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-width
     * @param {number} width
     */
    ideal_width(width) {
        wasm.devicevideotrackconstraints_ideal_width(this.__wbg_ptr, width);
    }
    /**
     * Sets an exact [`height`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-height
     * @param {number} height
     */
    exact_height(height) {
        wasm.devicevideotrackconstraints_exact_height(this.__wbg_ptr, height);
    }
    /**
     * Sets an ideal [`height`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-height
     * @param {number} height
     */
    ideal_height(height) {
        wasm.devicevideotrackconstraints_ideal_height(this.__wbg_ptr, height);
    }
    /**
     * Sets a range of a [`width`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-width
     * @param {number} min
     * @param {number} max
     */
    width_in_range(min, max) {
        wasm.devicevideotrackconstraints_width_in_range(this.__wbg_ptr, min, max);
    }
    /**
     * Sets a range of a [`height`][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-height
     * @param {number} min
     * @param {number} max
     */
    height_in_range(min, max) {
        wasm.devicevideotrackconstraints_height_in_range(this.__wbg_ptr, min, max);
    }
    /**
     * Sets an exact [facingMode][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
     * @param {FacingMode} facing_mode
     */
    exact_facing_mode(facing_mode) {
        wasm.devicevideotrackconstraints_exact_facing_mode(this.__wbg_ptr, facing_mode);
    }
    /**
     * Sets an ideal [facingMode][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
     * @param {FacingMode} facing_mode
     */
    ideal_facing_mode(facing_mode) {
        wasm.devicevideotrackconstraints_ideal_facing_mode(this.__wbg_ptr, facing_mode);
    }
    /**
     * Creates new [`DeviceVideoTrackConstraints`] with none constraints
     * configured.
     */
    constructor() {
        const ret = wasm.devicevideotrackconstraints_new();
        this.__wbg_ptr = ret >>> 0;
        DeviceVideoTrackConstraintsFinalization.register(this, this.__wbg_ptr, this);
        return this;
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
        wasm.devicevideotrackconstraints_device_id(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) DeviceVideoTrackConstraints.prototype[Symbol.dispose] = DeviceVideoTrackConstraints.prototype.free;

/**
 * Constraints applicable to display audio tracks (system audio capture).
 */
export class DisplayAudioTrackConstraints {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DisplayAudioTrackConstraintsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_displayaudiotrackconstraints_free(ptr, 0);
    }
    /**
     * Creates new [`DisplayAudioTrackConstraints`] with none constraints
     * configured.
     */
    constructor() {
        const ret = wasm.displayaudiotrackconstraints_new();
        this.__wbg_ptr = ret >>> 0;
        DisplayAudioTrackConstraintsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) DisplayAudioTrackConstraints.prototype[Symbol.dispose] = DisplayAudioTrackConstraints.prototype.free;

/**
 * Constraints applicable to video tracks sourced from a screen capturing.
 */
export class DisplayVideoTrackConstraints {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DisplayVideoTrackConstraintsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_displayvideotrackconstraints_free(ptr, 0);
    }
    /**
     * Sets an exact [width][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-width
     * @param {number} width
     */
    exact_width(width) {
        wasm.devicevideotrackconstraints_exact_width(this.__wbg_ptr, width);
    }
    /**
     * Sets an ideal [width][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-width
     * @param {number} width
     */
    ideal_width(width) {
        wasm.devicevideotrackconstraints_ideal_width(this.__wbg_ptr, width);
    }
    /**
     * Sets an exact [height][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-height
     * @param {number} height
     */
    exact_height(height) {
        wasm.devicevideotrackconstraints_exact_height(this.__wbg_ptr, height);
    }
    /**
     * Sets an ideal [height][1] constraint.
     *
     * [1]: https://tinyurl.com/w3-streams#def-constraint-height
     * @param {number} height
     */
    ideal_height(height) {
        wasm.devicevideotrackconstraints_ideal_height(this.__wbg_ptr, height);
    }
    /**
     * Sets an exact [frameRate][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
     * @param {number} frame_rate
     */
    exact_frame_rate(frame_rate) {
        wasm.displayvideotrackconstraints_exact_frame_rate(this.__wbg_ptr, frame_rate);
    }
    /**
     * Sets an ideal [frameRate][1] constraint.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
     * @param {number} frame_rate
     */
    ideal_frame_rate(frame_rate) {
        wasm.displayvideotrackconstraints_ideal_frame_rate(this.__wbg_ptr, frame_rate);
    }
    /**
     * Creates new [`DisplayVideoTrackConstraints`] with none constraints
     * configured.
     */
    constructor() {
        const ret = wasm.displayvideotrackconstraints_new();
        this.__wbg_ptr = ret >>> 0;
        DisplayVideoTrackConstraintsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) DisplayVideoTrackConstraints.prototype[Symbol.dispose] = DisplayVideoTrackConstraints.prototype.free;

/**
 * Exception thrown when cannot get info of available media devices.
 */
export class EnumerateDevicesException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EnumerateDevicesException.prototype);
        obj.__wbg_ptr = ptr;
        EnumerateDevicesExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EnumerateDevicesExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_enumeratedevicesexception_free(ptr, 0);
    }
    /**
     * Returns [`platform::Error`] causing this [`EnumerateDevicesException`].
     * @returns {Error}
     */
    cause() {
        const ret = wasm.enumeratedevicesexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`EnumerateDevicesException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.enumeratedevicesexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) EnumerateDevicesException.prototype[Symbol.dispose] = EnumerateDevicesException.prototype.free;

/**
 * Describes directions that a camera can face, as seen from a user's
 * perspective. Representation of a [VideoFacingModeEnum][1].
 *
 * [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
 * @enum {0 | 1 | 2 | 3}
 */
export const FacingMode = Object.freeze({
    /**
     * Facing towards a user (a self-view camera).
     */
    User: 0, "0": "User",
    /**
     * Facing away from a user (viewing the environment).
     */
    Environment: 1, "1": "Environment",
    /**
     * Facing to the left of a user.
     */
    Left: 2, "2": "Left",
    /**
     * Facing to the right of a user.
     */
    Right: 3, "3": "Right",
});

/**
 * Exception thrown when a string or some other data doesn't have an expected
 * format and cannot be parsed or processed.
 */
export class FormatException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FormatException.prototype);
        obj.__wbg_ptr = ptr;
        FormatExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FormatExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_formatexception_free(ptr, 0);
    }
    /**
     * Returns an error message describing of the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.formatexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) FormatException.prototype[Symbol.dispose] = FormatException.prototype.free;

/**
 * Jason's internal exception.
 *
 * This is either a programmatic error or some unexpected platform component
 * failure that cannot be handled in any way.
 */
export class InternalException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InternalException.prototype);
        obj.__wbg_ptr = ptr;
        InternalExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InternalExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_internalexception_free(ptr, 0);
    }
    /**
     * Returns [`platform::Error`] causing this [`RpcClientException`].
     * @returns {Error | undefined}
     */
    cause() {
        const ret = wasm.internalexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`InternalException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.internalexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns an error message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.internalexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) InternalException.prototype[Symbol.dispose] = InternalException.prototype.free;

/**
 * Exception thrown when cannot change output audio device ID.
 */
export class InvalidOutputAudioDeviceIdException {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InvalidOutputAudioDeviceIdExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invalidoutputaudiodeviceidexception_free(ptr, 0);
    }
    /**
     * Returns stacktrace of this [`InvalidOutputAudioDeviceIdException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.invalidoutputaudiodeviceidexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) InvalidOutputAudioDeviceIdException.prototype[Symbol.dispose] = InvalidOutputAudioDeviceIdException.prototype.free;

/**
 * General JS side library interface.
 *
 * Responsible for managing shared transports, local media and room
 * initialization.
 */
export class Jason {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JasonFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jason_free(ptr, 0);
    }
    /**
     * Closes the provided [`RoomHandle`].
     * @param {RoomHandle} room_to_delete
     */
    close_room(room_to_delete) {
        _assertClass(room_to_delete, RoomHandle);
        var ptr0 = room_to_delete.__destroy_into_raw();
        wasm.jason_close_room(this.__wbg_ptr, ptr0);
    }
    /**
     * Returns a [`MediaManagerHandle`].
     * @returns {MediaManagerHandle}
     */
    media_manager() {
        const ret = wasm.jason_media_manager(this.__wbg_ptr);
        return MediaManagerHandle.__wrap(ret);
    }
    /**
     * Notifies [`Jason`] about a network change event (interface switch or
     * similar).
     *
     * Drops and recreates active connections and schedules [ICE] restart after
     * reconnection.
     *
     * [ICE]: https://webrtcglossary.com/ice
     * @returns {Promise<any>}
     */
    network_changed() {
        const ret = wasm.jason_network_changed(this.__wbg_ptr);
        return ret;
    }
    /**
     * Instantiates a new [`Jason`] interface to interact with this library.
     */
    constructor() {
        const ret = wasm.jason_new();
        this.__wbg_ptr = ret >>> 0;
        JasonFinalization.register(this, this.__wbg_ptr, this);
        return this;
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
    /**
     * Creates a new `Room` and returns its [`RoomHandle`].
     * @returns {RoomHandle}
     */
    init_room() {
        const ret = wasm.jason_init_room(this.__wbg_ptr);
        return RoomHandle.__wrap(ret);
    }
}
if (Symbol.dispose) Jason.prototype[Symbol.dispose] = Jason.prototype.free;

/**
 * Exception thrown when accessing media devices.
 */
export class LocalMediaInitException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LocalMediaInitException.prototype);
        obj.__wbg_ptr = ptr;
        LocalMediaInitExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LocalMediaInitExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_localmediainitexception_free(ptr, 0);
    }
    /**
     * Returns concrete error kind of this [`LocalMediaInitException`].
     * @returns {LocalMediaInitExceptionKind}
     */
    kind() {
        const ret = wasm.localmediainitexception_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns [`platform::Error`] causing this [`LocalMediaInitException`].
     * @returns {Error | undefined}
     */
    cause() {
        const ret = wasm.localmediainitexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`LocalMediaInitException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.localmediainitexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns an error message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.localmediainitexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) LocalMediaInitException.prototype[Symbol.dispose] = LocalMediaInitException.prototype.free;

/**
 * Possible error kinds of a [`LocalMediaInitException`].
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const LocalMediaInitExceptionKind = Object.freeze({
    /**
     * Occurs if the [getUserMedia()][1] request failed.
     *
     * [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
     */
    GetUserMediaFailed: 0, "0": "GetUserMediaFailed",
    /**
     * Occurs if the [getUserMedia()][1] request failed on getting audio
     * track.
     *
     * [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
     */
    GetUserMediaAudioFailed: 1, "1": "GetUserMediaAudioFailed",
    /**
     * Occurs if the [getUserMedia()][1] request failed on getting video
     * track.
     *
     * [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
     */
    GetUserMediaVideoFailed: 2, "2": "GetUserMediaVideoFailed",
    /**
     * Occurs if the [getDisplayMedia()][1] request failed.
     *
     * [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
     */
    GetDisplayMediaFailed: 3, "3": "GetDisplayMediaFailed",
    /**
     * Occurs when local track is [`ended`][1] right after [getUserMedia()][2]
     * or [getDisplayMedia()][3] request.
     *
     * [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
     * [2]: https://tinyurl.com/rnxcavf
     * [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
     */
    LocalTrackIsEnded: 4, "4": "LocalTrackIsEnded",
});

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
        ptr = ptr >>> 0;
        const obj = Object.create(LocalMediaTrack.prototype);
        obj.__wbg_ptr = ptr;
        LocalMediaTrackFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LocalMediaTrackFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_localmediatrack_free(ptr, 0);
    }
    /**
     * Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
     * sourced from some device (webcam/microphone), or a
     * [`MediaSourceKind::Display`] if it's captured via
     * [MediaDevices.getDisplayMedia()][1].
     *
     * [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
     * @returns {MediaSourceKind}
     */
    media_source_kind() {
        const ret = wasm.localmediatrack_media_source_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Sets the provided function as the callback for the audio level changes
     * in this [`LocalMediaTrack`].
     *
     * # Errors
     *
     * If platform call errors.
     * @param {Function} cb
     */
    on_audio_level_changed(cb) {
        const ret = wasm.localmediatrack_on_audio_level_changed(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Indicates whether an `OnAudioLevelChangedCallback` is supported for this
     * [`LocalMediaTrack`].
     * @returns {boolean}
     */
    is_on_audio_level_available() {
        const ret = wasm.localmediatrack_is_on_audio_level_available(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Indicates whether auto gain control is enabled for this
     * [`LocalMediaTrack`].
     * @returns {Promise<any>}
     */
    is_auto_gain_control_enabled() {
        const ret = wasm.localmediatrack_is_auto_gain_control_enabled(this.__wbg_ptr);
        return ret;
    }
    /**
     * Indicates whether echo cancellation is enabled for this
     * [`LocalMediaTrack`].
     * @returns {Promise<any>}
     */
    is_echo_cancellation_enabled() {
        const ret = wasm.localmediatrack_is_echo_cancellation_enabled(this.__wbg_ptr);
        return ret;
    }
    /**
     * Indicates whether noise suppression is enabled for this
     * [`LocalMediaTrack`].
     * @returns {Promise<any>}
     */
    is_noise_suppression_enabled() {
        const ret = wasm.localmediatrack_is_noise_suppression_enabled(this.__wbg_ptr);
        return ret;
    }
    /**
     * Indicates whether this [`LocalMediaTrack`] supports audio processing
     * functions:
     * - [`LocalMediaTrack::is_noise_suppression_enabled()`]
     * - [`LocalMediaTrack::set_noise_suppression_enabled()`]
     * - [`LocalMediaTrack::get_noise_suppression_level()`]
     * - [`LocalMediaTrack::set_noise_suppression_level()`]
     * - [`LocalMediaTrack::is_echo_cancellation_enabled()`]
     * - [`LocalMediaTrack::set_echo_cancellation_enabled()`]
     * - [`LocalMediaTrack::is_auto_gain_control_enabled()`]
     * - [`LocalMediaTrack::set_auto_gain_control_enabled()`]
     * - [`LocalMediaTrack::is_high_pass_filter_enabled()`]
     * - [`LocalMediaTrack::set_high_pass_filter_enabled()`]
     * @returns {boolean}
     */
    is_audio_processing_available() {
        const ret = wasm.localmediatrack_is_audio_processing_available(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Toggles auto gain control for this [`LocalMediaTrack`].
     * @param {boolean} enabled
     * @returns {Promise<any>}
     */
    set_auto_gain_control_enabled(enabled) {
        const ret = wasm.localmediatrack_set_auto_gain_control_enabled(this.__wbg_ptr, enabled);
        return ret;
    }
    /**
     * Toggles acoustic echo cancellation for this [`LocalMediaTrack`].
     * @param {boolean} enabled
     * @returns {Promise<any>}
     */
    set_echo_cancellation_enabled(enabled) {
        const ret = wasm.localmediatrack_set_echo_cancellation_enabled(this.__wbg_ptr, enabled);
        return ret;
    }
    /**
     * Toggles noise suppression for this [`LocalMediaTrack`].
     * @param {boolean} enabled
     * @returns {Promise<any>}
     */
    set_noise_suppression_enabled(enabled) {
        const ret = wasm.localmediatrack_set_noise_suppression_enabled(this.__wbg_ptr, enabled);
        return ret;
    }
    /**
     * Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
     * audio track, or a [`MediaKind::Video`] if it represents a video track.
     * @returns {MediaKind}
     */
    kind() {
        const ret = wasm.localmediatrack_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
     * audio track, or a [`MediaKind::Video`] if it represents a video track.
     * @returns {Promise<any>}
     */
    state() {
        const ret = wasm.localmediatrack_state(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns the underlying [MediaStreamTrack][1].
     *
     * [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
     * @returns {MediaStreamTrack}
     */
    get_track() {
        const ret = wasm.localmediatrack_get_track(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) LocalMediaTrack.prototype[Symbol.dispose] = LocalMediaTrack.prototype.free;

/**
 * Representation of a [MediaDeviceInfo][1].
 *
 * [1]: https://w3.org/TR/mediacapture-streams#device-info
 */
export class MediaDeviceDetails {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MediaDeviceDetails.prototype);
        obj.__wbg_ptr = ptr;
        MediaDeviceDetailsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MediaDeviceDetailsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediadevicedetails_free(ptr, 0);
    }
    /**
     * Returns a kind of the represented device.
     *
     * This representation of [MediaDeviceInfo][1] is for input device ONLY.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#device-info
     * @returns {MediaDeviceKind}
     */
    kind() {
        const ret = wasm.mediadevicedetails_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns label describing the represented device (for example "External
     * USB Webcam").
     *
     * If the device has no associated label, then returns an empty string.
     * @returns {string}
     */
    label() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.mediadevicedetails_label(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
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
        const ret = wasm.mediadevicedetails_group_id(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * Returns a unique identifier for the represented device.
     * @returns {string}
     */
    device_id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.mediadevicedetails_device_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) MediaDeviceDetails.prototype[Symbol.dispose] = MediaDeviceDetails.prototype.free;

/**
 * [MediaDeviceInfo.kind][1] representation.
 *
 * [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
 * @enum {0 | 1 | 2}
 */
export const MediaDeviceKind = Object.freeze({
    /**
     * Audio input device (for example, a microphone).
     */
    AudioInput: 0, "0": "AudioInput",
    /**
     * Video input device (for example, a webcam).
     */
    VideoInput: 1, "1": "VideoInput",
    /**
     * Audio output device (for example, a pair of headphones).
     */
    AudioOutput: 2, "2": "AudioOutput",
});

/**
 * Media exchange direction of a `Track`.
 * @enum {0 | 1 | 2 | 3}
 */
export const MediaDirection = Object.freeze({
    /**
     * `Track` is enabled on recv and send sides.
     */
    SendRecv: 0, "0": "SendRecv",
    /**
     * `Track` is enabled on send side.
     */
    SendOnly: 1, "1": "SendOnly",
    /**
     * `Track` is enabled on recv side.
     */
    RecvOnly: 2, "2": "RecvOnly",
    /**
     * `Track` is disabled on both sides.
     */
    Inactive: 3, "3": "Inactive",
});

/**
 * [MediaStreamTrack.kind][1] representation.
 *
 * [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
 * @enum {0 | 1}
 */
export const MediaKind = Object.freeze({
    /**
     * Audio track.
     */
    Audio: 0, "0": "Audio",
    /**
     * Video track.
     */
    Video: 1, "1": "Video",
});

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
        ptr = ptr >>> 0;
        const obj = Object.create(MediaManagerHandle.prototype);
        obj.__wbg_ptr = ptr;
        MediaManagerHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MediaManagerHandleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediamanagerhandle_free(ptr, 0);
    }
    /**
     * Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
     * `free` was called on this [`MediaManagerHandle`], or on a [`Jason`] that
     * implicitly owns native object behind this [`MediaManagerHandle`].
     *
     * [`Jason`]: crate::api::Jason
     * [`StateError`]: crate::api::err::StateError
     * @param {Function} cb
     */
    on_device_change(cb) {
        const ret = wasm.mediamanagerhandle_on_device_change(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Returns a list of [`MediaDeviceDetails`] objects representing available
     * media input and output devices, such as microphones, cameras, and so
     * forth.
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        const ret = wasm.mediamanagerhandle_enumerate_devices(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns [`LocalMediaTrack`]s objects, built from the provided
     * [`MediaStreamSettings`].
     *
     * # Errors
     *
     * With a [`StateError`] if the underlying object has been disposed, e.g.
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
        const ret = wasm.mediamanagerhandle_init_local_tracks(this.__wbg_ptr, caps.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) MediaManagerHandle.prototype[Symbol.dispose] = MediaManagerHandle.prototype.free;

/**
 * Errors occurring in [`RoomHandle::set_local_media_settings()`][1] method.
 *
 * [1]: crate::api::RoomHandle::set_local_media_settings
 */
export class MediaSettingsUpdateException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MediaSettingsUpdateException.prototype);
        obj.__wbg_ptr = ptr;
        MediaSettingsUpdateExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MediaSettingsUpdateExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediasettingsupdateexception_free(ptr, 0);
    }
    /**
     * Returns whether media settings were successfully rolled back after new
     * settings application failed.
     * @returns {boolean}
     */
    rolled_back() {
        const ret = wasm.mediasettingsupdateexception_rolled_back(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns the original [`room::ChangeMediaStateError`] that was
     * encountered while updating local media settings.
     * @returns {any}
     */
    cause() {
        const ret = wasm.mediasettingsupdateexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns an error message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.mediasettingsupdateexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) MediaSettingsUpdateException.prototype[Symbol.dispose] = MediaSettingsUpdateException.prototype.free;

/**
 * Media source type.
 * @enum {0 | 1}
 */
export const MediaSourceKind = Object.freeze({
    /**
     * Media is sourced from some media device (webcam or microphone).
     */
    Device: 0, "0": "Device",
    /**
     * Media is obtained via screen capturing.
     */
    Display: 1, "1": "Display",
});

/**
 * Exception thrown when the requested media state transition could not be
 * performed.
 */
export class MediaStateTransitionException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MediaStateTransitionException.prototype);
        obj.__wbg_ptr = ptr;
        MediaStateTransitionExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MediaStateTransitionExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediastatetransitionexception_free(ptr, 0);
    }
    /**
     * Returns concrete error kind of this [`MediaStateTransitionException`].
     * @returns {MediaStateTransitionExceptionKind}
     */
    kind() {
        const ret = wasm.mediastatetransitionexception_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`MediaStateTransitionException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.mediastatetransitionexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns an error message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.mediastatetransitionexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) MediaStateTransitionException.prototype[Symbol.dispose] = MediaStateTransitionException.prototype.free;

/**
 * Kind of a [`MediaStateTransitionException`].
 * @enum {0 | 1}
 */
export const MediaStateTransitionExceptionKind = Object.freeze({
    /**
     * Media state of a [`Sender`] transits to an opposite of the requested
     * one.
     *
     * [`Sender`]: crate::peer::media::Sender
     */
    OppositeState: 0, "0": "OppositeState",
    /**
     * Requested state transition is not allowed by [`Sender`]'s settings.
     *
     * [`Sender`]: crate::peer::media::Sender
     */
    ProhibitedState: 1, "1": "ProhibitedState",
});

/**
 * [MediaStreamConstraints][1] wrapper.
 *
 * [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
 */
export class MediaStreamSettings {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MediaStreamSettingsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mediastreamsettings_free(ptr, 0);
    }
    /**
     * Specifies the nature and settings of a device audio
     * [MediaStreamTrack][1].
     *
     * [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
     * @param {DeviceAudioTrackConstraints} constraints
     */
    device_audio(constraints) {
        _assertClass(constraints, DeviceAudioTrackConstraints);
        var ptr0 = constraints.__destroy_into_raw();
        wasm.mediastreamsettings_device_audio(this.__wbg_ptr, ptr0);
    }
    /**
     * Set constraints that will be used to obtain a local video sourced from
     * a media device.
     * @param {DeviceVideoTrackConstraints} constraints
     */
    device_video(constraints) {
        _assertClass(constraints, DeviceVideoTrackConstraints);
        var ptr0 = constraints.__destroy_into_raw();
        wasm.mediastreamsettings_device_video(this.__wbg_ptr, ptr0);
    }
    /**
     * Specifies the nature and settings of a display audio
     * [MediaStreamTrack][1].
     *
     * Behaviour is platform dependent and there is no propper feature check.
     * It's known to only work in Chrome and Chrome-based browsers. It must
     * always be coupled with [`DisplayVideoTrackConstraints`], meaning that
     * system audio capture prompt is a part of the screen-sharing prompt, so
     * if you try to request `display_audio` without `display_video` the UA
     * will ask user for screen capture track anyway.
     *
     * It is also OS-dependent:
     * 1. Only `Chrome-tab` audio can be captured on macOS and Linux.
     * 2. Both `Chrome-tab` and `Entire screen` audio can be captured on
     *    Windows.
     *
     * [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
     * @param {DisplayAudioTrackConstraints} constraints
     */
    display_audio(constraints) {
        _assertClass(constraints, DisplayAudioTrackConstraints);
        var ptr0 = constraints.__destroy_into_raw();
        wasm.mediastreamsettings_display_audio(this.__wbg_ptr, ptr0);
    }
    /**
     * Set constraints that will be used to capture a local video from a user's
     * display.
     * @param {DisplayVideoTrackConstraints} constraints
     */
    display_video(constraints) {
        _assertClass(constraints, DisplayVideoTrackConstraints);
        var ptr0 = constraints.__destroy_into_raw();
        wasm.mediastreamsettings_display_video(this.__wbg_ptr, ptr0);
    }
    /**
     * Creates new [`MediaStreamSettings`] with none constraints configured.
     */
    constructor() {
        const ret = wasm.mediastreamsettings_new();
        this.__wbg_ptr = ret >>> 0;
        MediaStreamSettingsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) MediaStreamSettings.prototype[Symbol.dispose] = MediaStreamSettings.prototype.free;

/**
 * Liveness state of a [`MediaStreamTrack`][1].
 *
 * [1]: crate::platform::MediaStreamTrack
 * @enum {0 | 1}
 */
export const MediaStreamTrackState = Object.freeze({
    /**
     * Active track (the track's underlying media source is making a
     * best-effort attempt to provide a data in real time).
     */
    Live: 0, "0": "Live",
    /**
     * Ended track (the track's underlying media source is no longer providing
     * any data, and will never provide more data for this track).
     *
     * This is a final state.
     */
    Ended: 1, "1": "Ended",
});

/**
 * [`Connection`]'s state.
 */
export class MemberConnectionState {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MemberConnectionState.prototype);
        obj.__wbg_ptr = ptr;
        MemberConnectionStateFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MemberConnectionStateFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_memberconnectionstate_free(ptr, 0);
    }
    /**
     * Returns the [`Connection`]'s mode.
     * @returns {MemberConnectionStateKind}
     */
    kind() {
        const ret = wasm.memberconnectionstate_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns the [`Connection`]'s state associated with its mode.
     * @returns {any}
     */
    value() {
        const ret = wasm.memberconnectionstate_value(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) MemberConnectionState.prototype[Symbol.dispose] = MemberConnectionState.prototype.free;

/**
 * Possible kinds of [`Connection`]'s state.
 * @enum {0}
 */
export const MemberConnectionStateKind = Object.freeze({
    /**
     * [`Connection`]'s state is in [P2P mesh] mode.
     *
     * [P2P mesh]: https://webrtcglossary.com/mesh
     */
    P2P: 0, "0": "P2P",
});

/**
 * Exception thrown when cannot interact with microphone volume.
 */
export class MicVolumeException {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MicVolumeExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_micvolumeexception_free(ptr, 0);
    }
    /**
     * Returns the [`platform::Error`] causing this [`MicVolumeException`].
     * @returns {Error}
     */
    cause() {
        const ret = wasm.micvolumeexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`MicVolumeException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.micvolumeexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) MicVolumeException.prototype[Symbol.dispose] = MicVolumeException.prototype.free;

/**
 * Possible connection states of a [`PeerConnection`].
 * @enum {0 | 1 | 2 | 3 | 4 | 5}
 */
export const PeerConnectionState = Object.freeze({
    /**
     * At least one of the connection's [ICE] transports are in the
     * [`IceConnectionState::New`] state, and none of them are in one
     * of the following states: [`IceConnectionState::Checking`],
     * [`IceConnectionState::Failed`], or
     * [`IceConnectionState::Disconnected`], or all of the connection's
     * transports are in the [`IceConnectionState::Closed`] state.
     *
     * [ICE]: https://webrtcglossary.com/ice
     */
    New: 0, "0": "New",
    /**
     * One or more of the [ICE] transports are currently in the process of
     * establishing a connection; that is, their [`IceConnectionState`] is
     * either [`IceConnectionState::Checking`] or
     * [`IceConnectionState::Connected`], and no transports are in the
     * [`IceConnectionState::Failed`] state.
     *
     * [ICE]: https://webrtcglossary.com/ice
     */
    Connecting: 1, "1": "Connecting",
    /**
     * Every [ICE] transport used by the connection is either in use (state
     * [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`])
     * or is closed ([`IceConnectionState::Closed`]).
     *
     * In addition, at least one transport is either
     * [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`].
     *
     * [ICE]: https://webrtcglossary.com/ice
     */
    Connected: 2, "2": "Connected",
    /**
     * At least one of the [ICE] transports for the connection is in the
     * [`IceConnectionState::Disconnected`] state and none of the other
     * transports are in the state [`IceConnectionState::Failed`] or
     * [`IceConnectionState::Checking`].
     *
     * It's not a terminal state, and it can go back to
     * [`PeerConnectionState::Connecting`] and then
     * [`PeerConnectionState::Connected`] on its own.
     *
     * [ICE]: https://webrtcglossary.com/ice
     */
    Disconnected: 3, "3": "Disconnected",
    /**
     * One or more of the [ICE] transports on the connection is in the
     * [`IceConnectionState::Failed`] state.
     *
     * It's not a terminal state, and it can be fixed with [ICE] restart if
     * signalling connection is alive.
     *
     * [ICE]: https://webrtcglossary.com/ice
     */
    Failed: 4, "4": "Failed",
    /**
     * [`PeerConnection`] is closed.
     *
     * It's a terminal state.
     */
    Closed: 5, "5": "Closed",
});

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
        ptr = ptr >>> 0;
        const obj = Object.create(ReconnectHandle.prototype);
        obj.__wbg_ptr = ptr;
        ReconnectHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ReconnectHandleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reconnecthandle_free(ptr, 0);
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
        const ret = wasm.reconnecthandle_reconnect_with_delay(this.__wbg_ptr, delay_ms);
        return ret;
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
     * @param {number | null} [max_elapsed_time_ms]
     * @returns {Promise<any>}
     */
    reconnect_with_backoff(starting_delay_ms, multiplier, max_delay, max_elapsed_time_ms) {
        const ret = wasm.reconnecthandle_reconnect_with_backoff(this.__wbg_ptr, starting_delay_ms, multiplier, max_delay, isLikeNone(max_elapsed_time_ms) ? 0x100000001 : (max_elapsed_time_ms) >>> 0);
        return ret;
    }
}
if (Symbol.dispose) ReconnectHandle.prototype[Symbol.dispose] = ReconnectHandle.prototype.free;

/**
 * Wrapper around a received remote [MediaStreamTrack][1].
 *
 * [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
 */
export class RemoteMediaTrack {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RemoteMediaTrack.prototype);
        obj.__wbg_ptr = ptr;
        RemoteMediaTrackFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RemoteMediaTrackFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_remotemediatrack_free(ptr, 0);
    }
    /**
     * Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
     * @param {Function} cb
     */
    on_stopped(cb) {
        wasm.remotemediatrack_on_stopped(this.__wbg_ptr, cb);
    }
    /**
     * Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
     * @param {Function} cb
     */
    on_unmuted(cb) {
        wasm.remotemediatrack_on_unmuted(this.__wbg_ptr, cb);
    }
    /**
     * Returns the current general [`MediaDirection`] of this
     * [`RemoteMediaTrack`].
     * @returns {MediaDirection}
     */
    media_direction() {
        const ret = wasm.remotemediatrack_media_direction(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns a [`MediaSourceKind::Device`] if this [`RemoteMediaTrack`] is
     * sourced from some device (webcam/microphone), or a
     * [`MediaSourceKind::Display`] if it's captured via
     * [MediaDevices.getDisplayMedia()][1].
     *
     * [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
     * @returns {MediaSourceKind}
     */
    media_source_kind() {
        const ret = wasm.remotemediatrack_media_source_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
     * [`MediaDirection`] changes.
     * @param {Function} cb
     */
    on_media_direction_changed(cb) {
        wasm.remotemediatrack_on_media_direction_changed(this.__wbg_ptr, cb);
    }
    /**
     * Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
     * an audio track, or a [`MediaKind::Video`] if it represents a video
     * track.
     * @returns {MediaKind}
     */
    kind() {
        const ret = wasm.remotemediatrack_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Indicates whether this [`RemoteMediaTrack`] is muted.
     * @returns {boolean}
     */
    muted() {
        const ret = wasm.remotemediatrack_muted(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
     * @param {Function} cb
     */
    on_muted(cb) {
        wasm.remotemediatrack_on_muted(this.__wbg_ptr, cb);
    }
    /**
     * Returns the underlying [MediaStreamTrack][1].
     *
     * [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
     * @returns {MediaStreamTrack}
     */
    get_track() {
        const ret = wasm.remotemediatrack_get_track(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) RemoteMediaTrack.prototype[Symbol.dispose] = RemoteMediaTrack.prototype.free;

/**
 * The reason of why a `Room` was closed.
 *
 * Provided in a [`RoomCloseReason`]
 * @enum {0 | 1 | 2 | 3 | 4 | 5}
 */
export const RoomCloseKind = Object.freeze({
    /**
     * Unexpected client error.
     */
    InternalClientError: 0, "0": "InternalClientError",
    /**
     * Unexpected server error.
     */
    InternalServerError: 1, "1": "InternalServerError",
    /**
     * Room was normally closed by client via `Jason::close_room()`.
     */
    Finished: 2, "2": "Finished",
    /**
     * Connection has been inactive for a while and thus considered idle
     * by a server.
     */
    Idle: 3, "3": "Idle",
    /**
     * Establishing of connection with a server was rejected on server side.
     *
     * Most likely because of incorrect `Member` credentials.
     */
    Rejected: 4, "4": "Rejected",
    /**
     * Client was evicted on the server side.
     *
     * Usually this means that either `Member` or `Room` was deleted from the
     * server.
     */
    Evicted: 5, "5": "Evicted",
});

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
        ptr = ptr >>> 0;
        const obj = Object.create(RoomCloseReason.prototype);
        obj.__wbg_ptr = ptr;
        RoomCloseReasonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RoomCloseReasonFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_roomclosereason_free(ptr, 0);
    }
    /**
     * Indicates whether the [`Room`] was closed by server.
     *
     * [`Room`]: room::Room
     * @returns {boolean}
     */
    is_closed_by_server() {
        const ret = wasm.roomclosereason_is_closed_by_server(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns the [`Room`]'s close reason.
     *
     * [`Room`]: room::Room
     * @returns {RoomCloseKind}
     */
    reason() {
        const ret = wasm.roomclosereason_reason(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) RoomCloseReason.prototype[Symbol.dispose] = RoomCloseReason.prototype.free;

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
        ptr = ptr >>> 0;
        const obj = Object.create(RoomHandle.prototype);
        obj.__wbg_ptr = ptr;
        RoomHandleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RoomHandleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_roomhandle_free(ptr, 0);
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    mute_audio(source_kind) {
        const ret = wasm.roomhandle_mute_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    mute_video(source_kind) {
        const ret = wasm.roomhandle_mute_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    enable_audio(source_kind) {
        const ret = wasm.roomhandle_enable_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    enable_video(source_kind) {
        const ret = wasm.roomhandle_enable_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    unmute_audio(source_kind) {
        const ret = wasm.roomhandle_unmute_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    unmute_video(source_kind) {
        const ret = wasm.roomhandle_unmute_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    disable_audio(source_kind) {
        const ret = wasm.roomhandle_disable_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    disable_video(source_kind) {
        const ret = wasm.roomhandle_disable_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
        const ret = wasm.roomhandle_on_local_track(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
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
        const ret = wasm.roomhandle_on_new_connection(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
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
        const ret = wasm.roomhandle_on_connection_loss(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    enable_remote_audio(source_kind) {
        const ret = wasm.roomhandle_enable_remote_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    enable_remote_video(source_kind) {
        const ret = wasm.roomhandle_enable_remote_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    disable_remote_audio(source_kind) {
        const ret = wasm.roomhandle_disable_remote_audio(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
     * @param {MediaSourceKind | null} [source_kind]
     * @returns {Promise<any>}
     */
    disable_remote_video(source_kind) {
        const ret = wasm.roomhandle_disable_remote_video(this.__wbg_ptr, isLikeNone(source_kind) ? 2 : source_kind);
        return ret;
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
        const ret = wasm.roomhandle_on_failed_local_media(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
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
        const ret = wasm.roomhandle_set_local_media_settings(this.__wbg_ptr, settings.__wbg_ptr, stop_first, rollback_on_fail);
        return ret;
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
        const ret = wasm.roomhandle_join(this.__wbg_ptr, ptr0, len0);
        return ret;
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
     * [`RoomCloseReason`]: room::RoomCloseReasonImpl
     * [`StateError`]: crate::api::err::StateError
     * @param {Function} cb
     */
    on_close(cb) {
        const ret = wasm.roomhandle_on_close(this.__wbg_ptr, cb);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) RoomHandle.prototype[Symbol.dispose] = RoomHandle.prototype.free;

/**
 * Exceptions thrown from a RPC client that implements messaging with media
 * server.
 */
export class RpcClientException {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RpcClientException.prototype);
        obj.__wbg_ptr = ptr;
        RpcClientExceptionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RpcClientExceptionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rpcclientexception_free(ptr, 0);
    }
    /**
     * Returns concrete error kind of this [`RpcClientException`].
     * @returns {RpcClientExceptionKind}
     */
    kind() {
        const ret = wasm.rpcclientexception_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns [`platform::Error`] causing this [`RpcClientException`].
     * @returns {Error | undefined}
     */
    cause() {
        const ret = wasm.rpcclientexception_cause(this.__wbg_ptr);
        return ret;
    }
    /**
     * Returns stacktrace of this [`RpcClientException`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rpcclientexception_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns an error message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rpcclientexception_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) RpcClientException.prototype[Symbol.dispose] = RpcClientException.prototype.free;

/**
 * Possible error kinds of a [`RpcClientException`].
 * @enum {0 | 1 | 2}
 */
export const RpcClientExceptionKind = Object.freeze({
    /**
     * Connection with a server was lost.
     *
     * This usually means that some transport error occurred, so a client can
     * continue performing reconnecting attempts.
     */
    ConnectionLost: 0, "0": "ConnectionLost",
    /**
     * Could not authorize an RPC session.
     *
     * This usually means that authentication data a client provides is
     * obsolete.
     */
    AuthorizationFailed: 1, "1": "AuthorizationFailed",
    /**
     * RPC session has been finished. This is a terminal state.
     */
    SessionFinished: 2, "2": "SessionFinished",
});

/**
 * Error thrown when the operation wasn't allowed by the current state of the
 * object.
 */
export class StateError {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StateError.prototype);
        obj.__wbg_ptr = ptr;
        StateErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StateErrorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stateerror_free(ptr, 0);
    }
    /**
     * Returns native stacktrace of this [`StateError`].
     * @returns {string}
     */
    trace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.stateerror_trace(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns message describing the problem.
     * @returns {string}
     */
    message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.stateerror_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) StateError.prototype[Symbol.dispose] = StateError.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_debug_string_adfb662ae34724b6 = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_string_get_a2a31e16edf96e42 = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg__wbg_cb_unref_87dfb5aaa0cbcea7 = function(arg0) {
        arg0._wbg_cb_unref();
    };
    imports.wbg.__wbg_addEventListener_6a82629b3d430a48 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_addIceCandidate_a2f1c667d2083f21 = function(arg0, arg1) {
        const ret = arg0.addIceCandidate(arg1);
        return ret;
    };
    imports.wbg.__wbg_addTrack_d17de22dc51dc15c = function(arg0, arg1) {
        arg0.addTrack(arg1);
    };
    imports.wbg.__wbg_addTransceiver_fb01588fdbef142f = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.addTransceiver(getStringFromWasm0(arg1, arg2), arg3);
        return ret;
    };
    imports.wbg.__wbg_address_e6011f4c83d89660 = function(arg0, arg1) {
        const ret = arg1.address;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_applyConstraints_6c881a462688dee9 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.applyConstraints(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_3020136f7a2d6e44 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_abb4ff46ce38be40 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_candidate_4e3d572df5288462 = function(arg0, arg1) {
        const ret = arg1.candidate;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_candidate_d3b7043104cac524 = function(arg0) {
        const ret = arg0.candidate;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_clone_448790f2d6e18ecf = function(arg0) {
        const ret = arg0.clone();
        return ret;
    };
    imports.wbg.__wbg_close_1d59742d5bccce05 = function() { return handleError(function (arg0) {
        const ret = arg0.close();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_close_9cdab4afe1eeaf53 = function(arg0) {
        arg0.close();
    };
    imports.wbg.__wbg_close_ff2e6995683b2ad9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.close(arg1, getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_code_85a811fe6ca962be = function(arg0) {
        const ret = arg0.code;
        return ret;
    };
    imports.wbg.__wbg_connect_f28a2db518e02462 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.connect(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_connectionState_6cfc28903ca479ea = function(arg0) {
        const ret = arg0.connectionState;
        return (__wbindgen_enum_RtcPeerConnectionState.indexOf(ret) + 1 || 7) - 1;
    };
    imports.wbg.__wbg_connectionhandle_new = function(arg0) {
        const ret = ConnectionHandle.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_createAnalyser_9cbe07cbe76b4aa2 = function() { return handleError(function (arg0) {
        const ret = arg0.createAnalyser();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createAnswer_57fa5e0880a7b92a = function(arg0) {
        const ret = arg0.createAnswer();
        return ret;
    };
    imports.wbg.__wbg_createMediaStreamSource_bad9dbe1d85c3cb7 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.createMediaStreamSource(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createOffer_2a7c990705f87d09 = function(arg0, arg1) {
        const ret = arg0.createOffer(arg1);
        return ret;
    };
    imports.wbg.__wbg_crypto_574e78ad8b13b65f = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_data_8bf4ae669a78a688 = function(arg0) {
        const ret = arg0.data;
        return ret;
    };
    imports.wbg.__wbg_debug_9ad80675faf0c9cf = function(arg0, arg1, arg2, arg3) {
        console.debug(arg0, arg1, arg2, arg3);
    };
    imports.wbg.__wbg_deviceId_b59e94e1d736d75f = function(arg0, arg1) {
        const ret = arg1.deviceId;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_direction_348e9a15a2932e7c = function(arg0) {
        const ret = arg0.direction;
        return (__wbindgen_enum_RtcRtpTransceiverDirection.indexOf(ret) + 1 || 6) - 1;
    };
    imports.wbg.__wbg_disconnect_73648182b9afde22 = function() { return handleError(function (arg0) {
        arg0.disconnect();
    }, arguments) };
    imports.wbg.__wbg_done_62ea16af4ce34b24 = function(arg0) {
        const ret = arg0.done;
        return ret;
    };
    imports.wbg.__wbg_entries_ab58dcb3e812733b = function(arg0) {
        const ret = arg0.entries();
        return ret;
    };
    imports.wbg.__wbg_enumerateDevices_f2d3f89d7c5841a2 = function() { return handleError(function (arg0) {
        const ret = arg0.enumerateDevices();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_enumeratedevicesexception_new = function(arg0) {
        const ret = EnumerateDevicesException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_errorCode_4a894479ae47deac = function(arg0) {
        const ret = arg0.errorCode;
        return ret;
    };
    imports.wbg.__wbg_errorText_2a4382a8597cf5a7 = function(arg0, arg1) {
        const ret = arg1.errorText;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_error_7bc7d576a6aaf855 = function(arg0) {
        console.error(arg0);
    };
    imports.wbg.__wbg_error_ad1ecdacd1bb600d = function(arg0, arg1, arg2, arg3) {
        console.error(arg0, arg1, arg2, arg3);
    };
    imports.wbg.__wbg_formatexception_new = function(arg0) {
        const ret = FormatException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_from_29a8414a7a7cd19d = function(arg0) {
        const ret = Array.from(arg0);
        return ret;
    };
    imports.wbg.__wbg_getCapabilities_45b3208309f3f4aa = function(arg0, arg1) {
        const ret = RTCRtpSender.getCapabilities(getStringFromWasm0(arg0, arg1));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getCapabilities_76a672f0f5955cb7 = function(arg0) {
        const ret = arg0.getCapabilities();
        return ret;
    };
    imports.wbg.__wbg_getCapabilities_8d556ad8b5f96095 = function(arg0, arg1) {
        const ret = RTCRtpReceiver.getCapabilities(getStringFromWasm0(arg0, arg1));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getConstraints_667f8ff4ff1bcf10 = function(arg0) {
        const ret = arg0.getConstraints();
        return ret;
    };
    imports.wbg.__wbg_getDisplayMedia_d0594ad5fd0e8af4 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.getDisplayMedia(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getFloatTimeDomainData_a3f96ca6b29900d8 = function(arg0, arg1, arg2) {
        arg0.getFloatTimeDomainData(getArrayF32FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_getParameters_1f13bf7f8163b0cb = function(arg0) {
        const ret = arg0.getParameters();
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_b8f5dbd5f3995a9e = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_getSettings_955d12ff109fc0bc = function(arg0) {
        const ret = arg0.getSettings();
        return ret;
    };
    imports.wbg.__wbg_getStats_1516854022387ca8 = function(arg0) {
        const ret = arg0.getStats();
        return ret;
    };
    imports.wbg.__wbg_getTracks_75ecd4c89e587a44 = function(arg0) {
        const ret = arg0.getTracks();
        return ret;
    };
    imports.wbg.__wbg_getTransceivers_b58061e41459dc56 = function(arg0) {
        const ret = arg0.getTransceivers();
        return ret;
    };
    imports.wbg.__wbg_getUserMedia_6c26923b30317a2e = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.getUserMedia(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_6b7bd52aca3f9671 = function(arg0, arg1) {
        const ret = arg0[arg1 >>> 0];
        return ret;
    };
    imports.wbg.__wbg_get_af9dab7e9603ea93 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(arg0, arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_auto_gain_control_626f0d6bcf6b1c15 = function(arg0) {
        const ret = arg0.autoGainControl;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_auto_gain_control_9521e4278b209c9f = function(arg0) {
        const ret = arg0.autoGainControl;
        return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
    };
    imports.wbg.__wbg_get_channels_60862c7cb73b1141 = function(arg0) {
        const ret = arg0.channels;
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_get_clock_rate_48f75c8c1d3936c0 = function(arg0) {
        const ret = arg0.clockRate;
        return ret;
    };
    imports.wbg.__wbg_get_codecs_0f4d8fffdcb40fb5 = function(arg0) {
        const ret = arg0.codecs;
        return ret;
    };
    imports.wbg.__wbg_get_device_id_a36f21ad436d0ba8 = function(arg0, arg1) {
        const ret = arg1.deviceId;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_get_echo_cancellation_1db9e42345d04949 = function(arg0) {
        const ret = arg0.echoCancellation;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_echo_cancellation_56e8e60d0fa484ad = function(arg0) {
        const ret = arg0.echoCancellation;
        return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
    };
    imports.wbg.__wbg_get_encodings_fe780f80bacfb6f8 = function(arg0) {
        const ret = arg0.encodings;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_facing_mode_11328fce46561d23 = function(arg0, arg1) {
        const ret = arg1.facingMode;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_get_height_22f9882327e574cc = function(arg0) {
        const ret = arg0.height;
        return isLikeNone(ret) ? 0x100000001 : (ret) >> 0;
    };
    imports.wbg.__wbg_get_mime_type_6ce9a7327dde442a = function(arg0, arg1) {
        const ret = arg1.mimeType;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_get_noise_suppression_0e0f2fa2b2cdda54 = function(arg0) {
        const ret = arg0.noiseSuppression;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_noise_suppression_19da61519a077bd5 = function(arg0) {
        const ret = arg0.noiseSuppression;
        return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
    };
    imports.wbg.__wbg_get_sdp_fmtp_line_64348e6fd3a0f1f6 = function(arg0, arg1) {
        const ret = arg1.sdpFmtpLine;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_get_width_eb5936e59804955d = function(arg0) {
        const ret = arg0.width;
        return isLikeNone(ret) ? 0x100000001 : (ret) >> 0;
    };
    imports.wbg.__wbg_groupId_50ef6d039131ff19 = function(arg0, arg1) {
        const ret = arg1.groupId;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_iceConnectionState_aa66153d58e4b3b5 = function(arg0) {
        const ret = arg0.iceConnectionState;
        return (__wbindgen_enum_RtcIceConnectionState.indexOf(ret) + 1 || 8) - 1;
    };
    imports.wbg.__wbg_iceGatheringState_6b243c9b32142b25 = function(arg0) {
        const ret = arg0.iceGatheringState;
        return (__wbindgen_enum_RtcIceGatheringState.indexOf(ret) + 1 || 4) - 1;
    };
    imports.wbg.__wbg_id_def19f2e8d014164 = function(arg0, arg1) {
        const ret = arg1.id;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_info_b7fa8ce2e59d29c6 = function(arg0, arg1, arg2, arg3) {
        console.info(arg0, arg1, arg2, arg3);
    };
    imports.wbg.__wbg_instanceof_Error_3443650560328fa9 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Error;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_b5cf7783caa68180 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_internalexception_new = function(arg0) {
        const ret = InternalException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_is_928aa29d71e75457 = function(arg0, arg1) {
        const ret = Object.is(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_kind_4ff7cb96d9dbf2c2 = function(arg0) {
        const ret = arg0.kind;
        return (__wbindgen_enum_MediaDeviceKind.indexOf(ret) + 1 || 4) - 1;
    };
    imports.wbg.__wbg_kind_d12aea545977b91f = function(arg0, arg1) {
        const ret = arg1.kind;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_label_d42701c54a375486 = function(arg0, arg1) {
        const ret = arg1.label;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_length_22ac23eaec9d8053 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_length_d45040a40c570362 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_localmediainitexception_new = function(arg0) {
        const ret = LocalMediaInitException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_localmediatrack_new = function(arg0) {
        const ret = LocalMediaTrack.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_log_f614673762e98966 = function(arg0, arg1, arg2, arg3) {
        console.log(arg0, arg1, arg2, arg3);
    };
    imports.wbg.__wbg_mediaDevices_9cbe26ce22d56511 = function() { return handleError(function (arg0) {
        const ret = arg0.mediaDevices;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_mediadevicedetails_new = function(arg0) {
        const ret = MediaDeviceDetails.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_mediasettingsupdateexception_new = function(arg0) {
        const ret = MediaSettingsUpdateException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_mediastatetransitionexception_new = function(arg0) {
        const ret = MediaStateTransitionException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_memberconnectionstate_new = function(arg0) {
        const ret = MemberConnectionState.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_message_0305fa7903f4b3d9 = function(arg0) {
        const ret = arg0.message;
        return ret;
    };
    imports.wbg.__wbg_mid_8cfcd828b0e23a04 = function(arg0, arg1) {
        const ret = arg1.mid;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_msCrypto_a61aeb35a24c1329 = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_navigator_b49edef831236138 = function(arg0) {
        const ret = arg0.navigator;
        return ret;
    };
    imports.wbg.__wbg_new_1ba21ce319a06297 = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_new_25f239778d6112b9 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_new_5d252a0ded4f647e = function() { return handleError(function () {
        const ret = new MediaStream();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_5e542c992f14cb6f = function() { return handleError(function () {
        const ret = new lAudioContext();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_7c30d1f874652e62 = function() { return handleError(function (arg0, arg1) {
        const ret = new WebSocket(getStringFromWasm0(arg0, arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_new_df1173567d5ff028 = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_new_ff12d2b041fb48f1 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return wasm_bindgen__convert__closures_____invoke__h0b83e4d6cdffc65d(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return ret;
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_new_no_args_cb138f77cf6151ee = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_new_with_configuration_d8c11e79765332b1 = function() { return handleError(function (arg0) {
        const ret = new RTCPeerConnection(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_with_length_aa5eaf41d35235e5 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_next_3cfe5c0fe2a4cc53 = function() { return handleError(function (arg0) {
        const ret = arg0.next();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_node_905d3e251edff8a2 = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbg_now_8cf15d6e317793e1 = function(arg0) {
        const ret = arg0.now();
        return ret;
    };
    imports.wbg.__wbg_port_56163f3c7e40f54b = function(arg0) {
        const ret = arg0.port;
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_process_dc0fbacc7c1c06f7 = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_prototypesetcall_dfe9b766cdc1f1fd = function(arg0, arg1, arg2) {
        Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
    };
    imports.wbg.__wbg_push_7d9be8f38fc13975 = function(arg0, arg1) {
        const ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_9b549dfce8865860 = function(arg0) {
        const ret = arg0.queueMicrotask;
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_fca69f5bfad613a5 = function(arg0) {
        queueMicrotask(arg0);
    };
    imports.wbg.__wbg_randomFillSync_ac0988aba3254290 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_readyState_614515d18e053673 = function(arg0) {
        const ret = arg0.readyState;
        return (__wbindgen_enum_MediaStreamTrackState.indexOf(ret) + 1 || 3) - 1;
    };
    imports.wbg.__wbg_reason_d4eb9e40592438c2 = function(arg0, arg1) {
        const ret = arg1.reason;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_reconnecthandle_new = function(arg0) {
        const ret = ReconnectHandle.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_remotemediatrack_new = function(arg0) {
        const ret = RemoteMediaTrack.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_removeEventListener_565e273024b68b75 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_replaceTrack_bfebedd67ade9031 = function(arg0, arg1) {
        const ret = arg0.replaceTrack(arg1);
        return ret;
    };
    imports.wbg.__wbg_require_60cc747a6bc5215a = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_resolve_fd5bfbaa4ce36e1e = function(arg0) {
        const ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_roomclosereason_new = function(arg0) {
        const ret = RoomCloseReason.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_rpcclientexception_new = function(arg0) {
        const ret = RpcClientException.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_sdpMLineIndex_9adf8b2b8500f868 = function(arg0) {
        const ret = arg0.sdpMLineIndex;
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_sdpMid_145e795ed39a533e = function(arg0, arg1) {
        const ret = arg1.sdpMid;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_sdp_41383fc549912e3c = function(arg0, arg1) {
        const ret = arg1.sdp;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_send_7cc36bb628044281 = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.send(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_sender_d77a74938e41026f = function(arg0) {
        const ret = arg0.sender;
        return ret;
    };
    imports.wbg.__wbg_setCodecPreferences_b65c012d5208280b = function(arg0, arg1) {
        arg0.setCodecPreferences(arg1);
    };
    imports.wbg.__wbg_setLocalDescription_b2b733aef9d90b85 = function(arg0, arg1) {
        const ret = arg0.setLocalDescription(arg1);
        return ret;
    };
    imports.wbg.__wbg_setParameters_0dfc2ec1abc25547 = function(arg0, arg1) {
        const ret = arg0.setParameters(arg1);
        return ret;
    };
    imports.wbg.__wbg_setRemoteDescription_2678c3c1d5e054e5 = function(arg0, arg1) {
        const ret = arg0.setRemoteDescription(arg1);
        return ret;
    };
    imports.wbg.__wbg_setTimeout_06477c23d31efef1 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.setTimeout(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_4308bc4cf8a29876 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = Reflect.set(arg0, arg1, arg2, arg3);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_active_8e1f0e253038c475 = function(arg0, arg1) {
        arg0.active = arg1 !== 0;
    };
    imports.wbg.__wbg_set_audio_05d7278e0f8f36e1 = function(arg0, arg1) {
        arg0.audio = arg1;
    };
    imports.wbg.__wbg_set_audio_be0d6b299225bb44 = function(arg0, arg1) {
        arg0.audio = arg1;
    };
    imports.wbg.__wbg_set_auto_gain_control_f02c3a364ac7faa1 = function(arg0, arg1) {
        arg0.autoGainControl = arg1;
    };
    imports.wbg.__wbg_set_bundle_policy_f46eb5712a928dcb = function(arg0, arg1) {
        arg0.bundlePolicy = __wbindgen_enum_RtcBundlePolicy[arg1];
    };
    imports.wbg.__wbg_set_candidate_a300596d8311ab98 = function(arg0, arg1, arg2) {
        arg0.candidate = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_credential_477c08b06ae8e601 = function(arg0, arg1, arg2) {
        arg0.credential = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_device_id_47a63585ba7b83bf = function(arg0, arg1) {
        arg0.deviceId = arg1;
    };
    imports.wbg.__wbg_set_direction_40db73bad6d7aa8b = function(arg0, arg1) {
        arg0.direction = __wbindgen_enum_RtcRtpTransceiverDirection[arg1];
    };
    imports.wbg.__wbg_set_direction_44029f65e31fce1e = function(arg0, arg1) {
        arg0.direction = __wbindgen_enum_RtcRtpTransceiverDirection[arg1];
    };
    imports.wbg.__wbg_set_echo_cancellation_7493a67f2cd94967 = function(arg0, arg1) {
        arg0.echoCancellation = arg1;
    };
    imports.wbg.__wbg_set_enabled_179d2e1a43831d69 = function(arg0, arg1) {
        arg0.enabled = arg1 !== 0;
    };
    imports.wbg.__wbg_set_exact_307dce597de352c3 = function(arg0, arg1) {
        arg0.exact = arg1;
    };
    imports.wbg.__wbg_set_exact_53955cb8747fe462 = function(arg0, arg1) {
        arg0.exact = arg1;
    };
    imports.wbg.__wbg_set_exact_b58aef11a3390a5b = function(arg0, arg1) {
        arg0.exact = arg1 !== 0;
    };
    imports.wbg.__wbg_set_facing_mode_bdb8d769cb695131 = function(arg0, arg1) {
        arg0.facingMode = arg1;
    };
    imports.wbg.__wbg_set_fftSize_cbdd39b28a923496 = function(arg0, arg1) {
        arg0.fftSize = arg1 >>> 0;
    };
    imports.wbg.__wbg_set_frame_rate_dc0a11064476efbf = function(arg0, arg1) {
        arg0.frameRate = arg1;
    };
    imports.wbg.__wbg_set_height_c040b9a4303a22a3 = function(arg0, arg1) {
        arg0.height = arg1;
    };
    imports.wbg.__wbg_set_ice_restart_ca781acbf5f6efd0 = function(arg0, arg1) {
        arg0.iceRestart = arg1 !== 0;
    };
    imports.wbg.__wbg_set_ice_servers_7aa5a25622397c52 = function(arg0, arg1) {
        arg0.iceServers = arg1;
    };
    imports.wbg.__wbg_set_ice_transport_policy_44ce7e2c210224f2 = function(arg0, arg1) {
        arg0.iceTransportPolicy = __wbindgen_enum_RtcIceTransportPolicy[arg1];
    };
    imports.wbg.__wbg_set_ideal_160733c6b17f1e99 = function(arg0, arg1) {
        arg0.ideal = arg1;
    };
    imports.wbg.__wbg_set_ideal_8b6259dd933cf4e9 = function(arg0, arg1) {
        arg0.ideal = arg1 !== 0;
    };
    imports.wbg.__wbg_set_ideal_987e9b503d264087 = function(arg0, arg1) {
        arg0.ideal = arg1;
    };
    imports.wbg.__wbg_set_max_06cf4578a933aeec = function(arg0, arg1) {
        arg0.max = arg1;
    };
    imports.wbg.__wbg_set_max_bitrate_0bcb0ae45817b4ca = function(arg0, arg1) {
        arg0.maxBitrate = arg1 >>> 0;
    };
    imports.wbg.__wbg_set_min_43141d2248389d19 = function(arg0, arg1) {
        arg0.min = arg1;
    };
    imports.wbg.__wbg_set_noise_suppression_f585baa39bd281a7 = function(arg0, arg1) {
        arg0.noiseSuppression = arg1;
    };
    imports.wbg.__wbg_set_rid_33ca2623aa2cabe2 = function(arg0, arg1, arg2) {
        arg0.rid = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_scalability_mode_7314e7820ebe34f6 = function(arg0, arg1, arg2) {
        arg0.scalabilityMode = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_scale_resolution_down_by_c7d56857404c2856 = function(arg0, arg1) {
        arg0.scaleResolutionDownBy = arg1;
    };
    imports.wbg.__wbg_set_sdp_8a58fb4588ae8dfe = function(arg0, arg1, arg2) {
        arg0.sdp = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_sdp_m_line_index_bdca6129097ebe22 = function(arg0, arg1) {
        arg0.sdpMLineIndex = arg1 === 0xFFFFFF ? undefined : arg1;
    };
    imports.wbg.__wbg_set_sdp_mid_0e314047c91cf316 = function(arg0, arg1, arg2) {
        arg0.sdpMid = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_send_encodings_4f06eee89dd79c28 = function(arg0, arg1) {
        arg0.sendEncodings = arg1;
    };
    imports.wbg.__wbg_set_type_966bfe79c94c1a20 = function(arg0, arg1) {
        arg0.type = __wbindgen_enum_RtcSdpType[arg1];
    };
    imports.wbg.__wbg_set_urls_99ab80e82adb48b8 = function(arg0, arg1) {
        arg0.urls = arg1;
    };
    imports.wbg.__wbg_set_username_baac2f5590f7d70e = function(arg0, arg1, arg2) {
        arg0.username = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_set_video_188907421e06874f = function(arg0, arg1) {
        arg0.video = arg1;
    };
    imports.wbg.__wbg_set_video_26d356cfa8f70f76 = function(arg0, arg1) {
        arg0.video = arg1;
    };
    imports.wbg.__wbg_set_width_cd3820302c94ee85 = function(arg0, arg1) {
        arg0.width = arg1;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_state_c28a610878aaf489 = function(arg0) {
        const ret = arg0.state;
        return (__wbindgen_enum_AudioContextState.indexOf(ret) + 1 || 4) - 1;
    };
    imports.wbg.__wbg_stateerror_new = function(arg0) {
        const ret = StateError.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_769e6b65d6557335 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_08f5a74c69739274 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_a8924b26aa92d024 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_stop_8a61abecefef6af3 = function(arg0) {
        arg0.stop();
    };
    imports.wbg.__wbg_stopped_876711cbf8215d10 = function(arg0) {
        const ret = arg0.stopped;
        return ret;
    };
    imports.wbg.__wbg_stringify_655a6390e1f5eb6b = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_subarray_845f2f5bce7d061a = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_then_429f7caf1026411d = function(arg0, arg1, arg2) {
        const ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbg_then_4f95312d68691235 = function(arg0, arg1) {
        const ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_toString_14b47ee7542a49ef = function(arg0) {
        const ret = arg0.toString();
        return ret;
    };
    imports.wbg.__wbg_track_e6b12dd9ffd0b550 = function(arg0) {
        const ret = arg0.track;
        return ret;
    };
    imports.wbg.__wbg_transceiver_44e972a31038ccab = function(arg0) {
        const ret = arg0.transceiver;
        return ret;
    };
    imports.wbg.__wbg_url_9797f73efeeb1f3a = function(arg0, arg1) {
        const ret = arg1.url;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_value_57b7b035e117f7ee = function(arg0) {
        const ret = arg0.value;
        return ret;
    };
    imports.wbg.__wbg_values_1ad28fccb59aff05 = function(arg0) {
        const ret = arg0.values();
        return ret;
    };
    imports.wbg.__wbg_versions_c01dfd4722a88165 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_warn_165ef4f6bcfc05e7 = function(arg0, arg1, arg2, arg3) {
        console.warn(arg0, arg1, arg2, arg3);
    };
    imports.wbg.__wbindgen_cast_117bb628e323592b = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("Event")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_2b0b9690d109ebb9 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("CloseEvent")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_cast_5303d45e8e56dad7 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("RTCTrackEvent")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_cast_63ae25a80759f348 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 913, function: Function { arguments: [Externref], shim_idx: 914, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__he26b962bd17816c3, wasm_bindgen__convert__closures_____invoke__h359418e947c31e42);
        return ret;
    };
    imports.wbg.__wbindgen_cast_8233b4249018f698 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("RTCPeerConnectionIceErrorEvent")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_cast_cb9088102bce6b30 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
        const ret = getArrayU8FromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
        // Cast intrinsic for `F64 -> Externref`.
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_cast_e5af57ee27707e50 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("MessageEvent")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_cast_f577ef516b22962f = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 471, function: Function { arguments: [NamedExternref("RTCPeerConnectionIceEvent")], shim_idx: 472, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h070e68a4935d8bfa, wasm_bindgen__convert__closures_____invoke__h03097a538188cbc5);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('medea_jason_bg.wasm?7779b292ab3665cfd17e8fe6259af63f399f981f3abdeee2993b47d4d85d5ee7', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;

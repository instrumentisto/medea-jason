Feature: Remote media disabling

  Scenario: Remote video track stops when disabled
    Given room with joined members Alice and Bob
    When Alice disables remote video
    Then Alice's remote device video track from Bob disables

  Scenario: Remote audio track stops when disabled
    Given room with joined members Alice and Bob
    When Alice disables remote audio
    Then Alice's remote audio track from Bob disables

  Scenario: `RemoteTrack.on_disabled()` fires when audio is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote audio
    Then `on_disabled` callback fires 1 time on Alice's remote audio track from Bob

  Scenario: `RemoteTrack.on_disabled()` fires when video is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote video
    Then `on_disabled` callback fires 1 time on Alice's remote device video track from Bob

  Scenario: Remote member disables video
    Given room with joined members Alice and Bob
    When Bob disables video and awaits it completes
    Then `on_disabled` callback fires 1 time on Alice's remote device video track from Bob

  Scenario: Remote member disables audio
    Given room with joined members Alice and Bob
    When Bob disables audio and awaits it completes
    Then `on_disabled` callback fires 1 time on Alice's remote audio track from Bob

  Scenario: Member disables audio receiving from concrete `Connection`
    Given room with joined members Alice and Bob and Carol with disabled video publishing
    When Alice disables audio receiving from Bob
    Then Alice's remote audio track from Bob disables
    And Alice's audio remote track from Carol is enabled

  Scenario: Member disables video receiving from concrete `Connection`
    Given room with joined members Alice and Bob and Carol with disabled audio publishing
    When Alice disables video receiving from Bob
    Then Alice's remote device video track from Bob disables
    And Alice's device video remote track from Carol is enabled

  Scenario: Member enables video receiving from concrete `Connection`
    Given room with joined members Alice and Bob
    When Alice disables video receiving from Bob
    And Alice enables video receiving from Bob
    Then Alice's device video remote track from Bob is enabled

  Scenario: Member enables audio receiving from concrete `Connection`
    Given room with joined members Alice and Bob
    When Alice disables audio receiving from Bob
    And Alice enables audio receiving from Bob
    Then Alice's audio remote track from Bob is enabled

  Scenario: Member disables remote audio via `Room` and enables concrete `Connection`'s remote audio
    Given room with joined members Alice and Bob and Carol with disabled video publishing
    When Alice disables remote audio
    And Alice enables audio receiving from Bob
    Then Alice's audio remote track from Bob is enabled
    And Alice's audio remote track from Carol is disabled

  Scenario: Member disables remote video via `Room` and enables concrete `Connection`'s remote video
    Given room with joined members Alice and Bob and Carol with disabled audio publishing
    When Alice disables remote video
    And Alice enables video receiving from Bob
    Then Alice's device video remote track from Bob is enabled
    And Alice's device video remote track from Carol is disabled

  Scenario: Member disables remote video from `Connection` and enables remote video via `Room`
    Given room with joined members Alice and Bob and Carol with disabled audio publishing
    When Alice disables video receiving from Bob
    And Alice enables remote video
    Then Alice's device video remote track from Bob is enabled
    Then Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote audio from `Connection` and enables remote audio via `Room`
    Given room with joined members Alice and Bob and Carol with disabled video publishing
    When Alice disables audio receiving from Bob
    And Alice enables remote audio
    Then Alice's audio remote track from Bob is enabled
    Then Alice's audio remote track from Carol is enabled

  Scenario: Member disables all `Connection`s audio receiving and enables it via `Room`
    Given room with joined members Alice and Bob and Carol with disabled video publishing
    When Alice disables audio receiving from Bob
    And Alice disables audio receiving from Carol
    And Alice enables remote audio
    Then Alice's audio remote track from Bob is enabled
    Then Alice's audio remote track from Carol is enabled

  Scenario: Member disables all `Connection`s video receiving and enables it via `Room`
    Given room with joined members Alice and Bob and Carol with disabled audio publishing
    When Alice disables video receiving from Bob
    And Alice disables video receiving from Carol
    And Alice enables remote video
    Then Alice's device video remote track from Bob is enabled
    Then Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote video via `Room` and enables all `Connection`s remote videos
    Given room with joined members Alice and Bob and Carol with disabled audio publishing
    When Alice disables remote video
    And Alice enables video receiving from Bob
    And Alice enables video receiving from Carol
    Then Alice's device video remote track from Bob is enabled
    And Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote audio via `Room` and enables all `Connection`s remote audios
    Given room with joined members Alice and Bob and Carol with disabled video publishing
    When Alice disables remote audio
    And Alice enables audio receiving from Bob
    And Alice enables audio receiving from Carol
    Then Alice's audio remote track from Bob is enabled
    And Alice's audio remote track from Carol is enabled

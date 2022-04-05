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

  Scenario: Remote member disables audio by ConnectionHandle
    Given room with joined members Alice and Bob and Carol
    When Alice disables audio receive from Bob
    Then Alice's remote audio track from Bob disables
    And Alice's audio remote track from Carol is enabled

  Scenario: Remote member disables video by ConnectionHandle
    Given room with joined members Alice and Bob and Carol
    When Alice disables video receive from Bob
    Then Alice's remote device video track from Bob disables
    And Alice's device video remote track from Carol is enabled

  Scenario: Remote member enables video by ConnectionHandle
    Given room with joined members Alice and Bob
    When Alice disables video receive from Bob
    And Alice enables video receive from Bob
    Then Alice's device video remote track from Bob is enabled

  Scenario: Remote member enables audio by ConnectionHandle
    Given room with joined members Alice and Bob
    When Alice disables audio receive from Bob
    And Alice enables audio receive from Bob
    Then Alice's audio remote track from Bob is enabled

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

  Scenario: Member disables audio receive from concrete Connection
    Given room with joined members Alice and Bob and Carol
    When Alice disables audio receive from Bob
    Then Alice's remote audio track from Bob disables
    And Alice's audio remote track from Carol is enabled

  Scenario: Member disables video receive from concrete Connection
    Given room with joined members Alice and Bob and Carol
    When Alice disables video receive from Bob
    Then Alice's remote device video track from Bob disables
    And Alice's device video remote track from Carol is enabled

  Scenario: Member enables video receive from concrete Connection
    Given room with joined members Alice and Bob
    When Alice disables video receive from Bob
    And Alice enables video receive from Bob
    Then Alice's device video remote track from Bob is enabled

  Scenario: Member enables audio receive from concrete Connection
    Given room with joined members Alice and Bob
    When Alice disables audio receive from Bob
    And Alice enables audio receive from Bob
    Then Alice's audio remote track from Bob is enabled

  Scenario: Member disables remote audio via Room and enables concrete Connection's remote audio
    Given room with joined members Alice and Bob and Carol
    When Alice disables remote audio
    And Alice enables audio receive from Bob
    Then Alice's audio remote track from Bob is enabled
    And Alice's audio remote track from Carol is disabled

  Scenario: Member disables remote video via Room and enables concrete Connection's remote video
    Given room with joined members Alice and Bob and Carol
    When Alice disables remote video
    And Alice enables video receive from Bob
    Then Alice's device video remote track from Bob is enabled
    And Alice's device video remote track from Carol is disabled

  Scenario: Member disables remote video from Connection and enables remote video via Room
    Given room with joined members Alice and Bob and Carol
    When Alice disables video receive from Bob
    And Alice enables remote video
    Then Alice's device video remote track from Bob is enabled
    Then Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote audio from Connection and enables remote audio via Room
    Given room with joined members Alice and Bob and Carol
    When Alice disables audio receive from Bob
    And Alice enables remote audio
    Then Alice's audio remote track from Bob is enabled
    Then Alice's audio remote track from Carol is enabled

  Scenario: Member disables all Connections audio receive and enables it via Room
    Given room with joined members Alice and Bob and Carol
    When Alice disables audio receive from Bob
    And Alice disables audio receive from Carol
    And Alice enables remote audio
    Then Alice's audio remote track from Bob is enabled
    Then Alice's audio remote track from Carol is enabled

  Scenario: Member disables all Connections video receive and enables it via Room
    Given room with joined members Alice and Bob and Carol
    When Alice disables video receive from Bob
    And Alice disables video receive from Carol
    And Alice enables remote video
    Then Alice's device video remote track from Bob is enabled
    Then Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote video via Room and enables all Connection remote videos
    Given room with joined members Alice and Bob and Carol
    When Alice disables remote video
    And Alice enables video receive from Bob
    And Alice enables video receive from Carol
    Then Alice's device video remote track from Bob is enabled
    And Alice's device video remote track from Carol is enabled

  Scenario: Member disables remote audio via Room and enables all Connections remote audios
    Given room with joined members Alice and Bob and Carol
    When Alice disables remote audio
    And Alice enables audio receive from Bob
    And Alice enables audio receive from Carol
    Then Alice's audio remote track from Bob is enabled
    And Alice's audio remote track from Carol is enabled

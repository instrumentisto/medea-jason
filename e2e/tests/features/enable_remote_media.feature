Feature: Enable remote media

  @both
  Scenario: `RemoteMediaTrack.on_enabled()` fires when video is enabled
    Given room with joined member Bob
    Given joined member Alice with disabled video playing
    When Alice enables remote video
    Then `on_enabled` callback fires 1 time on Alice's remote device video track from Bob

  @both
  Scenario: `RemoteMediaTrack.on_enabled()` fires when audio is enabled
    Given room with joined member Bob
    Given joined member Alice with disabled audio playing
    When Alice enables remote audio
    Then `on_enabled` callback fires 1 time on Alice's remote audio track from Bob

  @both
  Scenario: `RemoteMediaTrack.on_enabled()` doesn't fire when track is created
    Given room with joined member Alice
    And member Bob
    When Bob joins the room
    Then `on_enabled` callback fires 0 times on Alice's remote audio track from Bob
    And `on_enabled` callback fires 0 times on Bob's remote audio track from Alice
    And `on_enabled` callback fires 0 times on Alice's remote device video track from Bob
    And `on_enabled` callback fires 0 times on Bob's remote device video track from Alice

  @both
  Scenario: Remote member enables video
    Given room with joined member Alice
    And joined member Bob
    When Bob disables video and awaits it completes
    And Bob enables video and awaits it completes
    Then `on_enabled` callback fires 1 time on Alice's remote device video track from Bob

  @both
  Scenario: Remote member enables audio
    Given room with joined member Alice
    And joined member Bob
    When Bob disables audio and awaits it completes
    And Bob enables audio and awaits it completes
    Then `on_enabled` callback fires 1 time on Alice's remote audio track from Bob

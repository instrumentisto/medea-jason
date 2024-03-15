Feature: Enable remote media

  Scenario: Enabling remote video works
    Given room with joined member Bob
    Given joined member Alice with disabled video playing
    When Alice enables remote video
    Then Alice's device video remote track from Bob is enabled

  Scenario: Enabling remote device video works
    Given room with joined member Bob
    Given joined member Alice with disabled video playing
    When Alice enables remote device video
    Then Alice's device video remote track from Bob is enabled

  Scenario: Enabling remote audio works
    Given room with joined member Bob
    Given joined member Alice with disabled audio playing
    When Alice enables remote audio
    Then Alice's audio remote track from Bob is enabled

  @mesh
  Scenario: `RemoteMediaTrack.on_enabled()` doesn't fire when track is created
    Given room with joined member Alice
    And member Bob
    When Bob joins the room
    Then `on_enabled` callback fires 0 times on Alice's remote audio track from Bob
    And `on_enabled` callback fires 0 times on Alice's remote device video track from Bob
    And `on_enabled` callback fires 0 times on Bob's remote audio track from Alice
    And `on_enabled` callback fires 0 times on Bob's remote device video track from Alice

  @mesh
  Scenario: Remote member enables video
    Given room with joined member Alice
    And joined member Bob
    When Bob disables video and awaits it completes
    And Bob enables video and awaits it completes
    Then `on_enabled` callback fires 1 times on Alice's remote device video track from Bob

  @mesh
  Scenario: Remote member enables audio
    Given room with joined member Alice
    And joined member Bob
    When Bob disables audio and awaits it completes
    And Bob enables audio and awaits it completes
    Then `on_enabled` callback fires 1 times on Alice's remote audio track from Bob

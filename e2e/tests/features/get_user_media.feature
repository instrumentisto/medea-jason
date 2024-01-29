Feature: `getUserMedia()` requests

  Scenario: Member joins Room and its `getUserMedia()` errors
    Given room with member Alice with disabled audio publishing
    And Alice's `getUserMedia()` errors
    And joined member Bob
    When Alice joins the room
    Then Alice's `Room.on_failed_local_stream()` fires 1 time

  Scenario: Member tries to enable media publishing and its `getUserMedia()` errors
    Given room with joined member Alice and Bob with disabled media publishing
    And Alice's `getUserMedia()` errors
    When Alice enables video and awaits it errors
    Then Alice's `Room.on_failed_local_stream()` fires 1 time

  Scenario: Member tries to enable audio and video and its `getUserMedia()` errors
    Given room with joined member Alice and Bob with disabled media publishing
    And Alice's `getUserMedia()` errors
    When Alice enables video and awaits it errors
    When Alice enables audio and awaits it errors
    Then Alice doesn't have live local tracks

  Scenario: Latency in `getUserMedia()` request
    Given room with joined member Alice and Bob
    When Alice switches device with latency
    And Alice disables video and awaits it completes
    Then `on_disabled` callback fires 1 time on Bob's remote device video track from Alice

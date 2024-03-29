Feature: Apply method of Control API

  Scenario: Remove member with `Apply` method
    Given room with joined member Alice and Bob
    When Control API removes Alice with `Apply` method
    Then Bob's connection with Alice closes

  Scenario: Remove connected member, recreate him and reconnect
    Given room with joined member Alice and Bob
    When Control API removes Alice with `Apply` method
    And Control API creates member Alice with `Apply` method
    And Control API interconnects Alice and Bob with `Apply` method
    And Alice joins the room
    Then Bob's video from Alice has `SendRecv` direction
    And Bob's audio from Alice has `SendRecv` direction

  Scenario: Interconnect members with `Apply` method
    Given room with joined member Alice and Bob with no WebRTC endpoints
    When Control API interconnects Alice and Bob with `Apply` method
    Then Alice receives connection with Bob
    And Bob receives connection with Alice

  Scenario: `OnJoin` callback fires on interconnection with `Apply` method
    Given room with joined member Alice and Bob with no WebRTC endpoints
    When Control API interconnects Alice and Bob with `Apply` method
    Then Control API sends `OnJoin` callback for member Alice

  Scenario: `Room.on_close()` fires when room is removed with `Apply` method
    Given room with joined member Alice
    When Control API removes Alice with `Apply` method
    Then Alice's `on_close` room's callback fires with `Evicted` reason

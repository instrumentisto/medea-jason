Feature: Apply method of Control API

  Scenario: Remove member with `Apply` method
    Given room with joined member Alice and Bob
    When Control API removes Alice with `Apply` method
    Then Bob's connection with Alice closes

  Scenario: Interconnect members with `Apply` method
    Given room with joined member Alice and Bob with no WebRTC endpoints
    When Control API interconnects Alice and Bob with `Apply` method
    Then Alice receives connection with Bob
    And Bob receives connection with Alice
Feature: Apply method of Control API

  Scenario: Remove member with `Apply` method
    Given room with joined member Alice and Bob
    When Control API removes Alice with `Apply` method
    Then Bob's connection with Alice closes
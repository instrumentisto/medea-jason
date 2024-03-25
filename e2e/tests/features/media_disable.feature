Feature: Media enabling/disabling

  Scenario: Member disables video during call
    Given room with joined members Alice and Bob
    And Alice's device video remote track from Bob is enabled
    When Bob disables video and awaits it completes
    Then Alice's device video remote track from Bob is disabled
    And Alice's audio remote track from Bob is enabled

  Scenario: Member disables audio during call
    Given room with joined members Alice and Bob
    And Alice's audio remote track from Bob is enabled
    When Bob disables audio and awaits it completes
    Then Alice's audio remote track from Bob is disabled
    And Alice's device video remote track from Bob is enabled

  Scenario: Member disables video before call
    Given room with joined member Alice
    And member Bob with disabled video publishing
    When Bob joins the room
    Then Alice doesn't have live device video remote track from Bob
    And Alice's audio remote track from Bob is enabled

  Scenario: Member disables audio before call
    Given room with joined member Alice
    And member Bob with disabled audio publishing
    When Bob joins the room
    Then Alice doesn't have live audio remote track from Bob
    And Alice's device video remote track from Bob is enabled

  Scenario: Member enables audio during call
    Given room with joined member Alice
    And member Bob with disabled audio publishing
    When Bob joins the room
    Then Alice receives connection with Bob
    And Bob receives connection with Alice
    And Alice's device video remote track from Bob is enabled
    And Alice doesn't have live audio remote track from Bob
    When Bob enables audio and awaits it completes
    Then Alice's audio remote track from Bob is enabled

  Scenario: Member enables video during call
    Given room with joined member Alice
    And member Bob with disabled video publishing
    When Bob joins the room
    Then Alice receives connection with Bob
    And Bob receives connection with Alice
    And Alice's audio remote track from Bob is enabled
    And Alice doesn't have live device video remote track from Bob
    When Bob enables video and awaits it completes
    Then Alice's device video remote track from Bob is enabled

  Scenario: Local track is dropped when video is disabled
    Given room with joined members Alice and Bob
    When Bob disables video and awaits it completes
    Then Bob's device video local track is stopped

  Scenario: Local track is dropped when audio is disabled
    Given room with joined members Alice and Bob
    When Bob disables audio and awaits it completes
    Then Bob's audio local track is stopped

  Scenario: Member starts enabling video and instantly disables it
    Given room with joined members Alice and Bob
    And Bob's `getUserMedia()` request has added latency
    When Bob disables video and ignores the result
    And Bob frees all local tracks
    And Bob enables video and ignores the result
    And Bob disables video and awaits it completes
    Then Alice's device video remote track from Bob is disabled

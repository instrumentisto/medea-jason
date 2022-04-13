Feature: RemoteMediaTrack media direction

  Scenario: Media direction is RecvOnly when audio sending is disabled
    Given room with joined members Alice and Bob
    When Bob disables audio and awaits it completes
    Then Alice's audio from Bob has SendOnly direction

  Scenario: Media direction is SendOnly when audio receiving is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote audio
    Then Alice's audio from Bob has SendOnly direction

  Scenario: Media direction is Inactive when audio receiving and sending is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote audio
    And Bob disables audio and awaits it completes
    Then Alice's audio from Bob has Inactive direction

  Scenario: Media direction is SendOnly when audio receiving is enabled after disabling
    Given room with joined members Alice and Bob
    When Alice disables remote audio
    And Alice enables remote audio
    Then Alice's audio from Bob has SendOnly direction

  Scenario: Media direction is SendOnly when audio sending is enabled after disabling
    Given room with joined members Alice and Bob
    When Bob disables audio and awaits it completes
    And Bob enables audio and awaits it completes
    Then Alice's audio from Bob has RecvOnly direction

  Scenario: Media direction is SendRecv when audio receiving and sending is enabled after disabling
    Given room with joined members Alice and Bob
    When Bob disables audio and awaits it completes
    And Bob enables audio and awaits it completes
    And Alice disables remote audio
    And Alice enables remote audio
    Then Alice's audio from Bob has SendRecv direction

  Scenario: Media direction is RecvOnly when video sending is disabled
    Given room with joined members Alice and Bob
    When Bob disables video and awaits it completes
    Then Alice's video from Bob has SendOnly direction

  Scenario: Media direction is SendOnly when video receiving is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote video
    Then Alice's video from Bob has SendOnly direction

  Scenario: Media direction is Inactive when video receiving and sending is disabled
    Given room with joined members Alice and Bob
    When Alice disables remote video
    And Bob disables video and awaits it completes
    Then Alice's video from Bob has Inactive direction

  Scenario: Media direction is SendOnly when video receiving is enabled after disabling
    Given room with joined members Alice and Bob
    When Alice disables remote video
    And Alice enables remote video
    Then Alice's video from Bob has SendOnly direction

  Scenario: Media direction is SendOnly when video sending is enabled after disabling
    Given room with joined members Alice and Bob
    When Bob disables video and awaits it completes
    And Bob enables video and awaits it completes
    Then Alice's video from Bob has RecvOnly direction

  Scenario: Media direction is SendRecv when video receiving and sending is enabled after disabling
    Given room with joined members Alice and Bob
    When Bob disables video and awaits it completes
    And Bob enables video and awaits it completes
    And Alice disables remote video
    And Alice enables remote video
    Then Alice's video from Bob has SendRecv direction

  Scenario: Media direction is SendRecv before any media updates
    Given room with joined members Alice and Bob
    Then Alice's video from Bob has SendRecv direction
    And Alice's audio from Bob has SendRecv direction
    And Bob's video from Alice has SendRecv direction
    And Bob's audio from Alice has SendRecv direction

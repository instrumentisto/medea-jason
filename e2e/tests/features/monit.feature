Feature: Register metrics

  Scenario: metrics: rooms
    When I check metrics
    Then response code is `200`
    And response contains `rooms` metrics

  Scenario: metrics: members
    When I check metrics
    Then response code is `200`
    And response contains `members` metrics

  Scenario: metrics: peers
    Given room with joined member Alice and Bob
    When I check metrics
    Then response code is `200`
    And response contains `peers` metrics

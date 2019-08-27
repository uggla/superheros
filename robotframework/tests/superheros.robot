*** Settings ***
Library  Collections
Library  String
Library  RequestsLibrary
Library  OperatingSystem

Suite Teardown  Delete All Sessions

*** Test Cases ***
Get Comics id
    Create Session  superheros  http://localhost:8088
    ${resp}=  Get Request  superheros  /Comics/2
    Should Be Equal As Strings  ${resp.status_code}  200
    ${json_from_file}=  evaluate  json.loads(open("/opt/robotframework/tests/Comics.json", "r").read())  json
    ${json_from_query}=  To Json  ${resp.content}
    Log to console  ${json_from_file}
    Log to console  ${json_from_query}
    Dictionaries Should Be Equal  ${json_from_file}  ${json_from_query}

Get Comics list
    Create Session  superheros  http://localhost:8088
    ${resp}=  Get Request  superheros  /Comics
    Should Be Equal As Strings  ${resp.status_code}  200
    ${json_from_file}=  evaluate  json.loads(open("/opt/robotframework/tests/ComicsList.json", "r").read())  json
    ${json_from_query}=  To Json  ${resp.content}
    Lists Should Be Equal  ${json_from_file}  ${json_from_query}

Get Characters
    Create Session  superheros  http://localhost:8088
    ${resp}=  Get Request  superheros  /Characters
    Should Be Equal As Strings  ${resp.status_code}  200
    ${json_from_file}=  evaluate  json.loads(open("/opt/robotframework/tests/Characters.json", "r").read())  json
    ${json_from_query}=  To Json  ${resp.content}
    Log to console  ${json_from_file}
    Log to console  ${json_from_query}
    Lists Should Be Equal  ${json_from_file}  ${json_from_query}

Get a bad route
    Create Session  superheros  http://localhost:8088
    ${resp}=  Get Request  superheros  /bidule
    Should Be Equal As Strings  ${resp.status_code}  404

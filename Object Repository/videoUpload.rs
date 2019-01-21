<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>videoUpload</name>
   <tag></tag>
   <elementGuidId>32c897f3-3555-479b-920f-f879e9cb6a4f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;title&quot;,
      &quot;value&quot;: &quot;Travel to Costa Rica&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;description&quot;,
      &quot;value&quot;: &quot;Test Description&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;privacy&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;tag&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;genre&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;userId&quot;,
      &quot;value&quot;: &quot;3&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;deviceType&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;deviceId&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;video&quot;,
      &quot;value&quot;: &quot;C:\\Users\\cinesoft\\Downloads\\Travel to - Costa Rica.mp4&quot;,
      &quot;type&quot;: &quot;File&quot;
    },
    {
      &quot;name&quot;: &quot;image&quot;,
      &quot;value&quot;: &quot;C:\\Users\\cinesoft\\Downloads\\Channel logos\\RealMadridTV(EN).jpg&quot;,
      &quot;type&quot;: &quot;File&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://13.234.26.238:8080/api_ogelle/videoUpload?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

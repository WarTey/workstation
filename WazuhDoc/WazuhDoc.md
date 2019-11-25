# WazuhDoc

**On Debian:**

To perform this procedure, the curl, apt-transport-https and lsb-release packages must be installed on your system. If they are not already present, install them using the commands below:

```
# apt-get update
# apt-get install curl apt-transport-https lsb-release gnupg2
```
##### Install the GPG key:

```
# curl -s https://packages.wazuh.com/key/GPG-KEY-WAZUH | apt-key add -

```

##### Add the repository:

```
# echo "deb https://packages.wazuh.com/3.x/apt/ stable main" | tee -a /etc/apt/sources.list.d/wazuh.list
```

##### Update the package information:

```
# apt-get update
```

## Installing the Wazuh server 
##### On your terminal, install the Wazuh manager:

```
# apt-get install wazuh-manager
```

##### Once the process is completed, you can check the service status with:

```
# systemctl status wazuh-manager
```

##### Installing the Wazuh API

NodeJS >= 4.6.1 is required in order to run the Wazuh API. If you do not have NodeJS installed or your version is older than 4.6.1, we recommend that you add the official NodeJS repository like this:

```
# curl -sL https://deb.nodesource.com/setup_8.x | bash -
```



If you are using Debian 7 (Wheezy) you must install NodeJS 6 using the command below: 

```
# curl -sL https://deb.nodesource.com/setup_6.x | bash -
```

and then, install NodeJS:

```
# apt-get install nodejs
```

##### Install the Wazuh API. It will update NodeJS if it is required:

```
# apt-get install wazuh-api
```

##### Once the process is complete, you can check the service status with:

```
# systemctl status wazuh-api
```
 
Note
Now that the Wazuh API is installed, check out the section Securing the Wazuh API to set up some additional settings.

##### (Optional) Disable the Wazuh updates:
It is recommended that the Wazuh repository be disabled in order to prevent accidental upgrades. To do this, use the following command:

```
# sed -i "s/^deb/#deb/" /etc/apt/sources.list.d/wazuh.list
# apt-get update
```



Alternately, you can set the package state to hold, which will stop updates (although you can still upgrade it manually using apt-get install).

```
# echo "wazuh-manager hold" | sudo dpkg --set-selections
# echo "wazuh-api hold" | sudo dpkg —set-selections
```



Securing the Wazuh API
By default, the communications between the Wazuh Kibana App and the Wazuh API are not encrypted. It is highly recommended that you secure the Wazuh API by following the steps below:

##### Enable HTTPS:

In order to enable HTTPS, you can generate your own certificate or generate it automatically by using the script /var/ossec/api/scripts/configure_api.sh.
Note
This script allows you to change the port used by the Wazuh API to handle the incoming HTTP requests. The port 55000 is used by default.
Change the default credentials:
The configure_api.sh script allows you to change the API’s user. If you did not use the script you can still change it as follows:

```
# cd /var/ossec/api/configuration/auth
# node htpasswd -Bc -C 10 user myUserName
```


By default, you can access the Wazuh API by typing user “foo” and password “bar”.
You will then need to restart the wazuh-api and wazuh-manager services for the change to take effect.

##### Bind to localhost:

If you do not need to access to the API externally, you should bind the API to localhost using the option *config.host* in the configuration file */var/ossec/api/configuration/config.js*.

Installing Filebeat
Filebeat is the tool on the Wazuh server that securely forwards alerts and archived events to Elasticsearch. To install it:

##### Add the Elastic repository and its GPG key:

```
# apt-get install curl apt-transport-https
# curl -s https://artifacts.elastic.co/GPG-KEY-elasticsearch | apt-key add -
# echo "deb https://artifacts.elastic.co/packages/7.x/apt stable main" | tee /etc/apt/sources.list.d/elastic-7.x.list
# apt-get update
```


##### Install Filebeat:

```
# apt-get install filebeat=7.3.2
```


##### Download the Filebeat config file from the Wazuh repository. This is pre-configured to forward Wazuh alerts to Elasticsearch:

```
# curl -so /etc/filebeat/filebeat.yml https://raw.githubusercontent.com/wazuh/wazuh/v3.10.2/extensions/filebeat/7.x/filebeat.yml
```


##### Download the alerts template for Elasticsearch:

```
# curl -so /etc/filebeat/wazuh-template.json https://raw.githubusercontent.com/wazuh/wazuh/v3.10.2/extensions/elasticsearch/7.x/wazuh-template.json
```

##### Download the Wazuh module for Filebeat:

```
# curl -s https://packages.wazuh.com/3.x/filebeat/wazuh-filebeat-0.1.tar.gz | sudo tar -xvz -C /usr/share/filebeat/module
```

Edit the file */etc/filebeat/filebeat.yml* and replace *YOUR\_ELASTIC\_SERVER\_IP* with the IP address or the hostname of the Elasticsearch server. For example:

output.elasticsearch.hosts: [‘http://YOUR\_ELASTIC\_SERVER\_IP:9200']

#### Enable and start the Filebeat service:

```
# systemctl daemon-reload
# systemctl enable filebeat.service
# systemctl start filebeat.service
```

Next steps
Once you have installed the manager, API and Filebeat, you are ready to install Elastic Stack.

## Preparation
#### Add the Elastic repository and its GPG key:

```
# apt-get install curl apt-transport-https
# curl -s https://artifacts.elastic.co/GPG-KEY-elasticsearch | apt-key add -
# echo "deb https://artifacts.elastic.co/packages/7.x/apt stable main" | tee /etc/apt/sources.list.d/elastic-7.x.list
# apt-get update
```

# Elasticsearch
Elasticsearch is a highly scalable full-text search and analytics engine. For more information, please see Elasticsearch.

#### Install the Elasticsearch package:

```
# apt-get install elasticsearch=7.3.2
```

Elasticsearch will only listen on the loopback interface (localhost) by default. Configure Elasticsearch to listen to a non-loopback address by editing the file */etc/elasticsearch/elasticsearch.yml* and uncommenting the setting *network.host*. Change the value to the IP you want to bind it to:

```
 network.host: <elasticsearch_ip>
```

Further configuration will be necessary after changing the network.host option. Add or edit (if commented) the following lines in the file /etc/elasticsearch/elasticsearch.yml:  

```  
node.name: <node_name>  
cluster.initial_master_nodes: ["<node_name>"]. 
```

#### Enable and start the Elasticsearch service:

```
# systemctl daemon-reload
# systemctl enable elasticsearch.service
# systemctl start elasticsearch.service
```

Once Elasticsearch is up and running, it is recommended to load the Filebeat template. Run the following command where Filebeat was installed:

```
# filebeat setup --index-management -E 
# setup.template.json.enabled=false
```

#### The Elasticsearch service listens on the default port 9200. You can make a simple check by making the following request:

```
# curl http://<elasticsearch_ip>:9200
```


Kibana
Kibana is a flexible and intuitive web interface for mining and visualizing the events and archives stored in Elasticsearch. Find more information at Kibana.

#### Install the Kibana package:

```
# apt-get install kibana=7.3.2
```

#### Install the Wazuh app plugin for Kibana:

* Install from URL:

```
# sudo -u kibana /usr/share/kibana/bin/kibana-plugin install https://packages.wazuh.com/wazuhapp/wazuhapp-3.10.2_7.3.2.zip
```

* Install from the package:

```
# sudo -u kibana /usr/share/kibana/bin/kibana-plugin install file:///path/wazuhapp-3.10.2_7.3.2.zip
```

The path should have read permissions for others. E.g: The directory /tmp/ accomplishes this.

Kibana will only listen on the loopback interface (localhost) by default, which means that it can be only accessed from the same machine. To access Kibana from the outside make it listen on its network IP by editing the file /etc/kibana/kibana.yml, uncomment the setting server.host, and change the value to:

```
server.host: « <kibana_ip> »
```

#### Configure the URLs of the Elasticsearch instances to use for all your queries. By editing the file */etc/kibana/kibana.yml*:

```
elasticsearch.hosts: [« http://<elasticsearch_ip>:9200 »]
```

#### Enable and start the Kibana service:

```
# systemctl daemon-reload
# systemctl enable kibana.service
# systemctl start kibana.service
```

### (Optional) Disable the Elasticsearch updates:

It is recommended that the Elasticsearch repository be disabled in order to prevent an upgrade to a newer Elastic Stack version due to the possibility of undoing changes with the App. To do this, use the following command:

```
# sed -i "s/^deb/#deb/" /etc/apt/sources.list.d/elastic-7.x.list
# apt-get update
```

Alternately, you can set the package state to hold, which will stop updates (although you can still upgrade it manually using apt-get install).

```
# echo "elasticsearch hold" | sudo dpkg --set-selections
# echo "kibana hold" | sudo dpkg —set-selections
```







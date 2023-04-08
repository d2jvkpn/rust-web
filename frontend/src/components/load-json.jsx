import React, { Component } from "react";
import { Button, message } from 'antd';

class LoadJSON extends Component {
  constructor (props) { // title, load
    super(props);

    this.state = {
    };
  }

  componentDidMount() {
    // console.log("~~~ componentDidMount");
  }

  componentDidUpdate() {
    // console.log("~~~ componentDidUpdate");
  }

  handle = (event) => {
    let files = event.target.files;
    if (files.length === 0 ) {
      return;
    }

    let target = files[0];
    if (target.size === 0) {
      message.warn("empty json file");
      return;
    }

    // target.size > maxSizeMB<<20 // file size it greater than 20MB
    if (!target.type.startsWith("application/json")) {
      message.warn("not a json file!");
      return;
    }

    let fr = new FileReader();
    fr.readAsText(target, "utf-8");

    fr.onload = (e) => {
      try {
        let data = JSON.parse(e.target.result);
        this.props.load(data);
      } catch (err) {
        message.warn(`failed to convert to json!`);
        console.log(`!!! failed to convert to json: ${err.message}`);
        return;
      };
    }
  }

  render() {
    return (<div>
      <input style={{display: "none"}} type="file" onChange={this.handle}/>

      <Button type="primary" size="small"
        onClick={(event) => event.target.parentElement.previousSibling.click()}
        title={this.props.title || "click to select"}
      >
        {this.props.title || "Load JSON"}
      </Button>
    </div>)
  }
}

export default LoadJSON;

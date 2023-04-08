import React, {Component} from "react";
import { Button } from 'antd'; // message

import { datetimeAsFilename } from "js/utils.js";

class ExportJSON extends Component {
  constructor (props) {
    super(props); // title || "", export()

    this.state = {
      prefix: this.props.prefix || "data",
    };
  }

  componentDidMount() {
    // console.log("~~~ componentDidMount");
  }

  componentDidUpdate() {
    // console.log("~~~ componentDidUpdate");
  }

  export = (data) => {
    const link = document.createElement("a");
    link.href = `data:text/json,${encodeURIComponent(JSON.stringify(data))}`;

    link.download = `${this.state.prefix}_${datetimeAsFilename()}.json`;
    link.click();
  }

  render() {
    return (<>
      {/*<button type="button" onClick={() => this.props.get(this.export)}>
        {this.props.title || "Export"}
      </button>*/}

      <Button type="primary" size="small" onClick={() => this.props.export(this.export)}>
        {this.props.title || "Export JSON"}
      </Button>
    </>)
  }
}

export default ExportJSON;

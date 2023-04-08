import React, { Component } from "react";
import { message } from "antd";

import BlankThumbnail from "img/blank_thumbnail.png";

class BrowseImage extends Component {
  constructor (props) { // (image, updateImage, width="160px", maxSizeMB=1);
    super(props);

    this.state = {
      imageData: null,
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
      message.warn("empty image file");
      return;
    }

    let maxSizeMB = this.props.maxSizeMB || 1;
    if (target.size > maxSizeMB<<20) {
      message.warn(`thumbnail file size exceeds ${maxSizeMB}MB`);
      return;
    }

    if (!["image/png", "image/jpeg", "image/webp"].includes(target.type)) {
      message.warn("please use a png, jpeg, webp image");
      return;
    }

    let fr = new FileReader();
    fr.readAsDataURL(target);

    fr.onload = () => {
      this.setState({imageData: fr.result});

      if (this.props.updateImage) {
        this.props.updateImage({
          file: target,
          ext: target.type.replace("image/", ""),
        });
      }
    }
  }

  render() {
    return (<div>
      <input style={{display: "none"}} type="file" onChange={this.handle}/>

      <img width={this.props.width || "160px"}
        title={this.props.title || "click to select"}
        alt={"click to select"}
        src={this.state.imageData || this.props.image || BlankThumbnail}
        onClick={(event) => event.target.previousSibling.click()}
      />
    </div>)
  }
}

export default BrowseImage;
